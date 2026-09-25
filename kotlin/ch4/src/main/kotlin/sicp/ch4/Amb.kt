// SPDX-License-Identifier: GPL-3.0-only
// The amb evaluator of section 4.3: the analyzed evaluator of 4.1.7 rebuilt
// over success continuations and an explicit backtracking engine, the book's
// 4.3.3 adapted to a host without re-entrant continuations. Every expression
// analyzes to an `AmbExec`, a procedure of the environment and a success
// continuation. Failure is the edition's `Fail` signal: a dead end throws
// [AmbFail], unwinding the host branch. The engine keeps the choice stack of
// resumable frames -- each `amb` choice point pushes a frame holding its
// remaining alternatives, environment, success continuation, and trail mark
// -- and the drive loop resumes the deepest pending frame, the book's
// depth-first chronological backtracking. Assignment pushes onto the undo
// trail; the loop unwinds the trail to the resumed frame's mark, so `set!`
// rolls back and the exercises' `permanent-set!` can simply skip the push.
// The driver exposes the first answer and a `tryAgain` entry point; every
// drive runs in a fresh Raise scope, so an object error anywhere in the
// search aborts it as a typed fault.
//
// One evaluator instance carries one live search (the frames and the trail);
// the exercises measure on fresh drivers. `Random` is the runtime's seeded
// xorshift64*; `ambDriver` threads a seed into the evaluator (D31) for the
// `ramb` extension of 4.50. The engine is deterministic: no threads -- the
// parallel-execute story belongs to 3.4.
//
// Host truth: a success continuation nest is as deep as its expression, and
// a resumed branch runs at the drive loop's depth, so backtracking costs no
// host stack; deep NON-tail object recursion still costs host frames, and
// the explicit-control evaluator of 5.4 is the real answer.

package sicp.ch4

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import kotlinx.collections.immutable.PersistentList
import sicp.runtime.AppE
import sicp.runtime.BeginE
import sicp.runtime.CondE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.IfE
import sicp.runtime.LambdaE
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.QuoteE
import sicp.runtime.Random
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.VPrimitive
import sicp.runtime.VProc
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.VarE
import sicp.runtime.callPrimitive
import sicp.runtime.isTrue

/**
 * The `Fail` signal: thrown by a dead end -- `(amb)`, a failed `require`,
 * an exhausted choice frame -- and caught by the drive loop, which resumes
 * the deepest pending choice frame.
 */
public object AmbFail : RuntimeException(null, null, false, false)

/** Thrown by the driver's success continuation to deliver an answer out
 * of the search; caught by the same drive loop. */
internal class AmbDeliver(
    val value: Value,
) : RuntimeException(null, null, false, false)

/**
 * The success continuation: the value just obtained. A dead end anywhere in
 * the branch signals [AmbFail] instead of calling a failure continuation.
 */
public typealias Succeed = context(Raise<SchemeError>)
(Value) -> Unit

/**
 * The execution procedure of the amb evaluator: the analyzed expression
 * awaiting an environment and a success continuation.
 */
public typealias AmbExec = context(Raise<SchemeError>)
(Env, Succeed) -> Unit

/** One resumable frame on the choice stack: the alternatives not yet
 * tried, where the search waits, and the trail mark to unwind to. */
private class ChoiceFrame(
    val alternatives: List<AmbExec>,
    var index: Int,
    val env: Env,
    val succeed: Succeed,
    val mark: Int,
)

/** One undo-trail entry: the binding a `set!` overwrote inside a branch. */
private class Undo(
    val env: Env,
    val name: String,
    val old: Value,
)

/**
 * The base amb evaluator. The book's clause chain lives in [analyze] and its
 * [baseAnalyze] fallback; the reserved `amb` head is recognized where the
 * book recognizes it, at the application head. Exercise extensions subclass
 * and widen [reservedClause].
 */
public open class AmbEvaluator(
    /** The global environment the driver set up. */
    public val global: Env,
    /** The seeded xorshift the driver threaded in (D31); the `ramb`
     * extension of 4.50 reads it. */
    public val random: Random? = null,
) {
    /** Choice events: one increment per alternative a choice point
     * delivers, from entry or from a failure resumption. */
    public var choicesTaken: Long = 0L
        private set

    /** Backtrack events: one increment per failure resumption that
     * delivers an alternative from a frame. */
    public var backtracks: Long = 0L
        private set

    /** The choice stack: the deepest pending frame is the last. */
    private val frames = ArrayDeque<ChoiceFrame>()

    /** The undo trail of `set!` effects, in commitment order. */
    private val trail = ArrayDeque<Undo>()

    /** Analyzed compound-procedure bodies, keyed by procedure identity. */
    private val bodies: MutableMap<VProc, AmbExec> = HashMap()

    /** Analyzes an already-parsed expression; the book's `analyze`. */
    public open fun analyze(expr: Expr): AmbExec = baseAnalyze(expr)

    /** Discards the live search: a new problem forgets the unexplored
     * alternatives of the old one and rolls its assignments back. */
    public fun clearSearch() {
        either { unwindTrailTo(0) }
        frames.clear()
    }

    /** Runs [exec] to the first answer, or returns null when the search
     * finds nothing. */
    context(r: Raise<SchemeError>)
    internal fun startDrive(
        exec: AmbExec,
        env: Env,
    ): Value? = keepDriving(exec, env)

    /** Resumes the pending search for its next answer. */
    context(r: Raise<SchemeError>)
    internal fun resumeDrive(): Value? = keepDriving(null, null)

    /** The drive loop: run the pending execution; on [AmbDeliver] the
     * answer leaves with the frames intact for `tryAgain`; on [AmbFail]
     * the deepest pending frame resumes, its trail rolled back. */
    context(r: Raise<SchemeError>)
    private fun keepDriving(
        first: AmbExec?,
        firstEnv: Env?,
    ): Value? {
        val sink: Succeed = { value -> throw AmbDeliver(value) }
        var pending: AmbExec? = first
        var pendingEnv: Env? = firstEnv
        var pendingSucceed: Succeed = sink
        while (true) {
            val exec = pending
            val env = pendingEnv
            if (exec != null && env != null) {
                try {
                    exec(env, pendingSucceed)
                } catch (deliver: AmbDeliver) {
                    return deliver.value
                } catch (deadEnd: AmbFail) {
                    // the branch died; resume the deepest pending frame
                }
                pending = null
            }
            val frame = frames.removeLastOrNull() ?: return null
            unwindTrailTo(frame.mark)
            if (frame.index >= frame.alternatives.size) continue
            val alternative = frame.alternatives[frame.index]
            frame.index += 1
            if (frame.index < frame.alternatives.size) frames.addLast(frame)
            choicesTaken++
            backtracks++
            pending = alternative
            pendingEnv = frame.env
            pendingSucceed = frame.succeed
        }
    }

    /** Rolls the undo trail back to [mark]; the loop calls this at every
     * resumption, so a `set!` inside an abandoned branch dies with it. */
    context(r: Raise<SchemeError>)
    private fun unwindTrailTo(mark: Int) {
        while (trail.size > mark) {
            val undo = trail.removeLast()
            undo.env.set(undo.name, undo.old)
        }
    }

    /** The book's clause chain, over execution procedures. */
    protected fun baseAnalyze(expr: Expr): AmbExec =
        when (expr) {
            is LitE -> succeedWith(expr.v)
            is QuoteE -> succeedWith(expr.datum)
            is VarE -> succeedVariable(expr.name)
            is SetE -> analyzedAssignment(expr.name, analyze(expr.value))
            is DefineE -> analyzedDefinition(expr, expr.value as? LambdaE)
            is IfE -> analyzedIf(analyze(expr.predicate), analyze(expr.consequent), analyze(expr.alternative))
            is LambdaE -> analyzedLambda(expr.params, expr.rest, expr.body, null)
            is BeginE -> analyzeSequence(expr.actions)
            is CondE -> analyze(condToIfChain(expr.clauses))
            is LetE -> analyze(letToCombinationExpr(expr))
            is AppE -> analyzedApplication(expr)
        }

    /** The book's `analyze-self-evaluating` and `analyze-quoted`: the value
     * is fixed at analysis time. */
    protected fun succeedWith(v: Value): AmbExec = { _, succeed -> succeed(v) }

    /** The book's `analyze-variable`; a lookup miss is a program bug, not a
     * nondeterministic failure, so it raises past the search. */
    protected fun succeedVariable(name: String): AmbExec =
        { env, succeed ->
            succeed(env.lookup(name))
        }

    /** The book's `analyze-definition`; a procedure definition names its
     * [VProc]. */
    protected fun analyzedDefinition(
        expr: DefineE,
        lambda: LambdaE?,
    ): AmbExec {
        val vproc =
            if (lambda != null) {
                analyzedLambda(lambda.params, lambda.rest, lambda.body, expr.name)
            } else {
                analyze(expr.value)
            }
        return { env, succeed ->
            vproc(env) { value ->
                env.define(expr.name, value)
                succeed(VSym("ok"))
            }
        }
    }

    /** The book's `analyze-lambda`: the body analyzed once, registered
     * under the procedure object this execution creates. */
    protected fun analyzedLambda(
        params: PersistentList<String>,
        rest: String?,
        body: PersistentList<Expr>,
        name: String?,
    ): AmbExec {
        val bproc = analyzeSequence(body)
        return { env, succeed ->
            val procedure = VProc(params, rest, body, env, name)
            bodies[procedure] = bproc
            succeed(procedure)
        }
    }

    /** The book's `analyze-if`: the predicate's branch choice keeps the
     * search's success continuation. */
    protected fun analyzedIf(
        pproc: AmbExec,
        cproc: AmbExec,
        aproc: AmbExec,
    ): AmbExec =
        { env, succeed ->
            pproc(env) { predValue ->
                if (isTrue(predValue)) cproc(env, succeed) else aproc(env, succeed)
            }
        }

    /** The book's `analyze-sequence`: the execution procedures composed at
     * analysis time, each stage's success continuation calling the next. */
    protected fun analyzeSequence(actions: PersistentList<Expr>): AmbExec {
        if (actions.isEmpty()) {
            return { _, _ -> fail(SchemeError.Parse("empty sequence")) }
        }
        var combined: AmbExec = analyze(actions.last())
        for (i in actions.size - 2 downTo 0) {
            val earlier = analyze(actions[i])
            val later = combined
            combined = { env, succeed ->
                earlier(env) { later(env, succeed) }
            }
        }
        return combined
    }

    /** The book's `analyze-assignment` with the undo trail: the new value
     * is committed with a trail entry the loop rolls back on backtrack. */
    protected fun analyzedAssignment(
        name: String,
        vproc: AmbExec,
    ): AmbExec =
        { env, succeed ->
            vproc(env) { value ->
                val oldValue = env.lookup(name)
                env.set(name, value)
                trail.addLast(Undo(env, name, oldValue))
                succeed(VSym("ok"))
            }
        }

    /** The reserved-head clause: the application forms the evaluator
     * treats as special. Returns null when the head names no reserved
     * form, and the application stays ordinary. */
    protected open fun reservedClause(expr: AppE): AmbExec? {
        val head = expr.operator as? VarE ?: return null
        return when (head.name) {
            "amb" -> analyzedAmb(analyzedOperands(expr))
            else -> null
        }
    }

    /** The operand execution procedures, analyzed once. */
    protected fun analyzedOperands(expr: AppE): List<AmbExec> = expr.operands.map { analyze(it) }

    /** The book's `analyze-application`. */
    protected fun analyzedApplication(expr: AppE): AmbExec {
        val reserved = reservedClause(expr)
        if (reserved != null) {
            return reserved
        }
        val fproc = analyze(expr.operator)
        val aprocs = analyzedOperands(expr)
        return { env, succeed ->
            fproc(env) { procedure ->
                deliverArgs(aprocs, 0, env, emptyList(), procedure, succeed)
            }
        }
    }

    /** The book's `get-args`: operands left to right, each success
     * continuation collecting one argument (4.46's order). The collected
     * list is rebuilt per branch, so a backtrack sees no stale argument. */
    context(r: Raise<SchemeError>)
    private fun deliverArgs(
        aprocs: List<AmbExec>,
        index: Int,
        env: Env,
        collected: List<Value>,
        procedure: Value,
        succeed: Succeed,
    ) {
        if (index >= aprocs.size) {
            executeApplication(procedure, collected, succeed)
            return
        }
        aprocs[index](env) { arg ->
            deliverArgs(aprocs, index + 1, env, collected + arg, procedure, succeed)
        }
    }

    /** The book's `execute-application`: a primitive succeeds with its
     * value; a compound procedure runs its analyzed body on the extended
     * frame; anything else is a program bug, raised past the search. */
    context(r: Raise<SchemeError>)
    protected fun executeApplication(
        procedure: Value,
        arguments: List<Value>,
        succeed: Succeed,
    ) {
        when (procedure) {
            is VPrimitive -> succeed(callPrimitive(procedure, arguments))
            is VProc -> applyCompound(procedure, arguments, succeed)
            else -> r.raise(SchemeError.NotApplicable(procedure))
        }
    }

    context(r: Raise<SchemeError>)
    private fun applyCompound(
        procedure: VProc,
        arguments: List<Value>,
        succeed: Succeed,
    ) {
        val body =
            bodies[procedure]
                ?: r.raise(SchemeError.MachineFault("unanalyzed procedure: $procedure"))
        val frame = Env.extend(procedure.params.toList(), arguments, procedure.env, procedure.rest)
        body(frame, succeed)
    }

    /** The book's `analyze-amb`: the choice point. Entry pushes the
     * resumable frame for the alternatives after the first and delivers
     * the first alternative at once; an empty `amb` fails outright. The
     * per-visit order goes through [visitOrder], which the `ramb`
     * extension shuffles. */
    protected open fun analyzedAmb(alternatives: List<AmbExec>): AmbExec =
        { env, succeed ->
            choicesTaken++
            deliverChoice(visitOrder(alternatives), env, succeed)
        }

    /** The per-visit ordering of a choice point's alternatives. */
    protected open fun visitOrder(alternatives: List<AmbExec>): List<AmbExec> = alternatives

    /** Delivers the first alternative of a fresh choice-point visit and
     * pushes the frame that resumes at the second; an empty choice fails. */
    context(r: Raise<SchemeError>)
    protected fun deliverChoice(
        alternatives: List<AmbExec>,
        env: Env,
        succeed: Succeed,
    ) {
        if (alternatives.isEmpty()) {
            throw AmbFail
        }
        frames.addLast(ChoiceFrame(alternatives, 1, env, succeed, trail.size))
        alternatives[0](env, succeed)
    }

    /** Pushes a guard frame that fires only when every frame above it is
     * exhausted: the `if-fail` extension delivers its alternative there,
     * with the guard's trail mark rolling the branch's assignments back. */
    protected fun pushFailGuard(
        resumption: AmbExec,
        env: Env,
        succeed: Succeed,
    ) {
        frames.addLast(ChoiceFrame(listOf(resumption), 0, env, succeed, trail.size))
    }
}

/**
 * The amb driver: the book's driver loop over a fixed environment. A
 * problem runs to its first answer; `tryAgain` resumes the deepest pending
 * choice frame for the next one; a new problem discards the unexplored
 * alternatives of the old one. The protocol rounds reproduce the book's
 * `;;; Amb-Eval input:` interaction; the value entry points serve the
 * exercises.
 */
public class AmbDriver
    internal constructor(
        /** The evaluator this driver drives; the counters live here. */
        public val evaluator: AmbEvaluator,
        private val env: Env,
        private val sink: OutputSink,
    ) {
        /** Every answer of [query], first through exhaustion. */
        context(r: Raise<SchemeError>)
        public fun solveAll(query: String): List<Value> {
            val answers = mutableListOf<Value>()
            var next = solve(query)
            while (next != null) {
                answers.add(next)
                next = tryAgain()
            }
            return answers
        }

        /** The first answer of [query], or null when the search finds
         * nothing. A new problem discards any pending search. */
        context(r: Raise<SchemeError>)
        public fun solve(query: String): Value? = runProblem(parseExpr(readProgram(query).singleOrParseFault()))

        /** The next answer of the pending problem, or null when the
         * choices are exhausted. */
        context(r: Raise<SchemeError>)
        public fun tryAgain(): Value? = evaluator.resumeDrive()

        /** Whether a problem is in progress: `try-again` before any
         * problem, or after its choices are exhausted, is the book's
         * "no current problem" round. */
        private var problemActive = false

        /** One driver-loop round: the input prompt with [text] echoed, then
         * the book's outcome lines, appended to the sink and returned. */
        public fun input(text: String): String {
            val before = sink.toString()
            sink.line(";;; Amb-Eval input:")
            sink.line(text)
            if (text.trim() == "try-again") {
                recordTryAgain()
            } else {
                recordNewProblem(text)
            }
            return sink.toString().removePrefix(before)
        }

        /** The `try-again` round of the protocol. */
        public fun tryAgainRound(): String {
            val before = sink.toString()
            recordTryAgain()
            return sink.toString().removePrefix(before)
        }

        /** The transcript the sink holds so far. */
        public fun transcript(): String = sink.toString()

        context(r: Raise<SchemeError>)
        private fun runProblem(expr: Expr): Value? {
            evaluator.clearSearch()
            return evaluator.startDrive(evaluator.analyze(expr), env)
        }

        private fun recordTryAgain() {
            if (!problemActive) {
                sink.line(";;; There is no current problem")
                return
            }
            val outcome = either { tryAgain() }
            when (outcome) {
                is Either.Left -> {
                    recordFault(outcome.value)
                }

                is Either.Right -> {
                    if (outcome.value == null) {
                        problemActive = false
                    }
                    recordAnswer(outcome.value)
                }
            }
        }

        private fun recordNewProblem(text: String) {
            problemActive = true
            sink.line(";;; Starting a new problem")
            val outcome = either { solveProblem(readProgram(text)) }
            when (outcome) {
                is Either.Left -> recordFault(outcome.value)
                is Either.Right -> recordAnswer(outcome.value.first, outcome.value.second)
            }
        }

        context(r: Raise<SchemeError>)
        private fun solveProblem(forms: List<Value>): Pair<Value?, Value> {
            val datum = forms.singleOrParseFault()
            return Pair(runProblem(parseExpr(datum)), datum)
        }

        private fun recordAnswer(
            answer: Value?,
            datum: Value? = null,
        ) {
            if (answer != null) {
                sink.line(";;; Amb-Eval value:")
                sink.line(printValue(answer))
                return
            }
            sink.line(";;; There are no more values of")
            sink.line(if (datum == null) "the pending problem" else printValue(datum))
        }

        private fun recordFault(e: SchemeError) {
            problemActive = false
            evaluator.clearSearch()
            sink.line("Error: ${formatError(e)}")
        }
    }

private fun List<Value>.singleOrParseFault(): Value =
    if (size == 1) first() else throw IllegalArgumentException("expected one form, got $size")

/**
 * A driver on a fresh global environment with [prelude]'s definitions
 * installed. [seed] threads the runtime's seeded xorshift into the
 * evaluator (D31); null keeps the engine deterministic without `ramb`.
 */
public fun ambDriver(
    evaluatorFactory: (Env, Random?) -> AmbEvaluator,
    prelude: String,
    seed: ULong? = null,
): AmbDriver {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    val random = seed?.let { value -> seededOrFault(value) }
    val evaluator = evaluatorFactory(env, random)
    installPrelude(evaluator, env, prelude)
    return AmbDriver(evaluator, env, sink)
}

private fun seededOrFault(seed: ULong): Random =
    when (val outcome = Random.seeded(seed)) {
        is Either.Left -> throw IllegalArgumentException("driver seed must be nonzero")
        is Either.Right -> outcome.value
    }

private fun installPrelude(
    evaluator: AmbEvaluator,
    env: Env,
    prelude: String,
) {
    either {
        for (datum in readProgram(prelude)) {
            installForm(evaluator, env, parseExpr(datum))
        }
    }.fold(
        { e -> throw IllegalStateException("driver prelude failed: ${formatError(e)}") },
        { },
    )
}

context(r: Raise<SchemeError>)
private fun installForm(
    evaluator: AmbEvaluator,
    env: Env,
    expr: Expr,
) {
    if (expr !is DefineE) {
        r.raise(SchemeError.Parse("driver prelude expects definitions"))
    }
    evaluator.analyze(expr)(env) { }
}
