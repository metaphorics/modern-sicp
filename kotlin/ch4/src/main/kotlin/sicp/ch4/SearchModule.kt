// SPDX-License-Identifier: GPL-3.0-only
package sicp.ch4

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.guest.Admission
import sicp.guest.AdmissionError
import sicp.guest.Assignment
import sicp.guest.Binary
import sicp.guest.Block
import sicp.guest.Break
import sicp.guest.Call
import sicp.guest.CallableReference
import sicp.guest.Cell
import sicp.guest.CheckedProgram
import sicp.guest.Continue
import sicp.guest.DataClass
import sicp.guest.DataObject
import sicp.guest.Destructure
import sicp.guest.Elvis
import sicp.guest.Env
import sicp.guest.Expression
import sicp.guest.ExpressionStatement
import sicp.guest.For
import sicp.guest.FunctionDecl
import sicp.guest.GValue
import sicp.guest.GuestError
import sicp.guest.If
import sicp.guest.Index
import sicp.guest.Is
import sicp.guest.Lambda
import sicp.guest.Literal
import sicp.guest.LocalProperty
import sicp.guest.Member
import sicp.guest.Mode
import sicp.guest.NO_POSITION
import sicp.guest.Name
import sicp.guest.OutputSink
import sicp.guest.PlainClass
import sicp.guest.Primitives
import sicp.guest.Program
import sicp.guest.Return
import sicp.guest.RunResult
import sicp.guest.SealedInterface
import sicp.guest.Span
import sicp.guest.Statement
import sicp.guest.StringTemplate
import sicp.guest.This
import sicp.guest.TopProperty
import sicp.guest.Unary
import sicp.guest.When
import sicp.guest.WhenBranch
import sicp.guest.While
import sicp.guest.checkedLiteralValue
import sicp.guest.valueEquals

/** One search run: the effect stream of every attempt in program order, the
 * first success's value (null when every attempt failed), and the choice
 * count (one per entered alternative). */
public class SearchRun(
    public val result: RunResult,
    public val choices: Long,
)

/** The search experiment of section 4.3, mode `Search`: choice commits left
 * to right, `demand` failure backtracks to the most recent untried
 * alternative, ordinary binding writes revert with their attempt while
 * `setPermanent` writes survive, and the run explores to exhaustion. */
public object SearchModule {
    /** Admits in [Mode.SEARCH] and runs; admission failures execute nothing. */
    public fun run(source: String): Either<AdmissionError, SearchRun> = either { execute(Admission.admitOrRaise(source, Mode.SEARCH)) }

    /** Runs only through the first [maxAnswers] successful entry answers.
     * Unlike taking the lines of an exhaustive run, this does not force the
     * rest of an unbounded generator. */
    public fun run(
        source: String,
        maxAnswers: Int,
    ): Either<AdmissionError, SearchRun> {
        require(maxAnswers > 0) { "maxAnswers must be positive" }
        return either { execute(Admission.admitOrRaise(source, Mode.SEARCH), maxAnswers) }
    }

    /** Stops after at most [maxAnswers] results or [maxChoices] entered
     * alternatives, whichever comes first. */
    public fun run(
        source: String,
        maxAnswers: Int,
        maxChoices: Int,
    ): Either<AdmissionError, SearchRun> {
        require(maxAnswers > 0 && maxChoices > 0) { "search limits must be positive" }
        return either { execute(Admission.admitOrRaise(source, Mode.SEARCH), maxAnswers, maxChoices.toLong()) }
    }

    private fun execute(
        checked: CheckedProgram,
        maxAnswers: Int? = null,
        maxChoices: Long? = null,
    ): SearchRun {
        val sink = OutputSink()
        val outcome: Either<GuestError, SearchState> = either { SearchEvaluator(checked, sink).explore(maxAnswers, maxChoices) }
        return outcome.fold(
            { error -> SearchRun(RunResult(sink.contents(), error, null), 0L) },
            { state -> SearchRun(RunResult(sink.contents(), null, state.firstAnswer), state.choices) },
        )
    }
}

internal class SearchState {
    var firstAnswer: GValue? = null
    var choices: Long = 0
    var answers: Long = 0
}

/** Success delivers one answer and the continuation that resumes the search
 * for the next one; failure requests the next untried alternative. */
private typealias Ok = (GValue, Fail) -> Unit

private typealias Fail = () -> Unit

internal class SearchEvaluator(
    private val checked: CheckedProgram,
    private val sink: OutputSink,
) {
    private val globals = Env.root()
    private val classTable = linkedMapOf<String, Evaluator.ClassShape>()
    private val closures = java.util.IdentityHashMap<GValue.VFunction, ClosureShape>()
    private val state = SearchState()
    private val returnContinuations = java.util.WeakHashMap<Env, Ok>()
    private val pendingContinuations = ArrayDeque<Pending>()
    private val permanent = java.util.IdentityHashMap<Cell, GValue>()
    private val mutations = mutableListOf<StoreState>()
    private val permanentMutations = mutableListOf<StoreState>()
    private var permanentDepth = 0
    private var random: GValue.VRandom? = null
    private var synchronousDepth = 0
    private var choiceLimit: Long? = null
    private var choiceHorizonReached = false

    private data class Pending(
        val permanentDepth: Int,
        val resume: Fail,
    )

    private class ChoiceHorizon : RuntimeException(null, null, false, false)

    private fun enqueue(resume: Fail) {
        pendingContinuations.addFirst(Pending(permanentDepth, resume))
    }

    private fun returnTo(env: Env): Ok? {
        var here: Env? = env
        while (here != null) {
            returnContinuations[here]?.let { return it }
            here = here.parent
        }
        return null
    }

    private class ClosureShape(
        val statements: List<Statement>?,
        val expression: Expression?,
        val parameters: List<String>,
        val definingEnv: Env,
    )

    context(r: Raise<GuestError>)
    fun explore(
        maxAnswers: Int? = null,
        maxChoices: Long? = null,
    ): SearchState {
        choiceLimit = maxChoices
        try {
            installDeclarations(checked.syntax)
            if (choiceHorizonReached) return state
            val main = globals.lookup("main")?.value
            if (main !is GValue.VFunction) {
                state.firstAnswer = GValue.VUnit
                return state
            }
            applyFunction(main, emptyList(), { value, more ->
                if (state.firstAnswer == null) state.firstAnswer = value
                state.answers++
                if (maxAnswers == null || state.answers < maxAnswers) more()
            }, {})
            drainPending(maxAnswers)
        } catch (_: ChoiceHorizon) {
            return state
        }
        return state
    }

    private fun drainPending(maxAnswers: Int? = null) {
        while (pendingContinuations.isNotEmpty() && !choiceHorizonReached &&
            (maxAnswers == null || state.answers < maxAnswers)
        ) {
            val pending = pendingContinuations.removeFirst()
            val previous = permanentDepth
            permanentDepth = pending.permanentDepth
            try {
                pending.resume()
            } finally {
                permanentDepth = previous
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun installDeclarations(program: Program) {
        for (declaration in checked.vocabulary + program.declarations) {
            when (declaration) {
                is SealedInterface -> {
                    classTable[declaration.name] =
                        Evaluator.ClassShape(declaration.name, false, false, emptyList(), emptyMap())
                }

                is DataClass -> {
                    classTable[declaration.name] =
                        Evaluator.ClassShape(declaration.name, true, false, declaration.properties, emptyMap())
                }

                is PlainClass -> {
                    classTable[declaration.name] =
                        Evaluator.ClassShape(
                            declaration.name,
                            false,
                            false,
                            declaration.properties,
                            declaration.methods.associateBy { it.name },
                        )
                }

                is DataObject -> {
                    classTable[declaration.name] = Evaluator.ClassShape(declaration.name, true, true, emptyList(), emptyMap())
                    globals.define(declaration.name, GValue.VObject(declaration.name, structural = true, fields = mutableMapOf()))
                }

                else -> {}
            }
        }
        for (declaration in program.declarations) {
            if (declaration is FunctionDecl) globals.define(declaration.name, functionValue(declaration, globals))
        }
        for (declaration in program.declarations) {
            if (choiceHorizonReached) return
            if (declaration is TopProperty) {
                eval(declaration.initializer, globals, { value, _ ->
                    globals.define(declaration.property.name, value)
                }, {})
                drainPending()
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun functionValue(
        declaration: FunctionDecl,
        definingEnv: Env,
    ): GValue.VFunction {
        val shape =
            ClosureShape(
                declaration.body
                    .let {
                        it as? Block
                    }?.statements,
                declaration.body.takeIf { it !is Block },
                declaration.parameters.map { it.name },
                definingEnv,
            )
        val fn =
            GValue.VFunction(declaration.name, declaration.parameters.size) { arguments ->
                var answer: GValue = GValue.VUnit
                runClosure(shape, arguments) { value, _ -> answer = value }
                answer
            }
        closures[fn] = shape
        return fn
    }

    context(r: Raise<GuestError>)
    private fun lambdaValueOf(
        statements: List<Statement>,
        parameters: List<String>,
        env: Env,
    ): GValue.VFunction {
        val shape = ClosureShape(statements, null, parameters, env)
        val fn =
            GValue.VFunction(null, parameters.size) { arguments ->
                var answer: GValue = GValue.VUnit
                runClosure(shape, arguments) { value, _ -> answer = value }
                answer
            }
        closures[fn] = shape
        return fn
    }

    /** Runs one closure in continuation style: the answers stream through
     * [ok], each with its own continuation. */
    context(r: Raise<GuestError>)
    private fun runClosure(
        shape: ClosureShape,
        arguments: List<GValue>,
        ok: Ok,
    ) {
        val env = Env.child(shape.definingEnv)
        returnContinuations[env] = ok
        for ((name, value) in shape.parameters.zip(arguments)) env.define(name, value)
        synchronousDepth++
        try {
            val statements = shape.statements
            if (statements == null) {
                eval(shape.expression ?: return, env, ok, {})
            } else {
                runStatements(statements, env, ok, {})
            }
        } finally {
            synchronousDepth--
        }
    }

    context(r: Raise<GuestError>)
    private fun runStatements(
        statements: List<Statement>,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        fun step(
            index: Int,
            currentFail: Fail,
            lastValue: GValue,
        ) {
            if (index == statements.size) {
                ok(lastValue, currentFail)
                return
            }
            execStatement(statements[index], env, { value, more -> step(index + 1, more, value) }, currentFail)
        }
        step(0, fail, GValue.VUnit)
    }

    // ---------- expressions ----------

    context(r: Raise<GuestError>)
    private fun eval(
        expression: Expression,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        when (expression) {
            is Literal -> {
                ok(checkedLiteralValue(expression, checked.types[expression]), fail)
            }

            is Name -> {
                ok(readName(expression.text, expression, env), fail)
            }

            is StringTemplate -> {
                evalTemplate(expression, env, ok, fail)
            }

            is Binary -> {
                evalBinary(expression, env, ok, fail)
            }

            is Unary -> {
                eval(
                    expression.operand,
                    env,
                    { value, more -> ok(Primitives.unary(expression.operator, value, expression.span), more) },
                    fail,
                )
            }

            is Elvis -> {
                eval(
                    expression.left,
                    env,
                    { value, more -> if (value is GValue.VNull) eval(expression.right, env, ok, fail) else ok(value, more) },
                    fail,
                )
            }

            is Is -> {
                eval(expression.value, env, { value, more ->
                    ok(
                        GValue.VBool(Primitives.isTypeValue(value, expression.type) != expression.negated),
                        more,
                    )
                }, fail)
            }

            is If -> {
                evalIf(expression, env, ok, fail)
            }

            is When -> {
                evalWhen(expression, env, ok, fail)
            }

            is Lambda -> {
                ok(lambdaValueOf(expression.body.statements, expression.parameters.map { it.name }, env), fail)
            }

            is Call -> {
                evalCall(expression, env, ok, fail)
            }

            is Member -> {
                evalMember(expression, env, ok, fail)
            }

            is Index -> {
                eval(expression.receiver, env, { target, _ ->
                    eval(expression.index, env, { index, more -> ok(Primitives.readIndex(target, index, expression.span), more) }, fail)
                }, fail)
            }

            is Block -> {
                evalBlock(expression, env, ok, fail)
            }

            is Return -> {
                val returnOk = returnTo(env) ?: r.raise(GuestError.ShapeFault(expression.span))
                val operand = expression.value
                if (operand == null) {
                    returnOk(GValue.VUnit, fail)
                } else {
                    eval(operand, env, { value, more -> returnOk(value, more) }, fail)
                }
            }

            is This -> {
                ok(readName("this", expression, env), fail)
            }

            is CallableReference -> {
                ok(readName(expression.name, expression, env), fail)
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun readName(
        name: String,
        at: Expression,
        env: Env,
    ): GValue {
        val cell = env.lookup(name) ?: globals.lookup(name) ?: r.raise(GuestError.UnassignedRead(at.span))
        val value = cell.value
        if (value is GValue.VUnassigned) r.raise(GuestError.UnassignedRead(at.span))
        return value
    }

    context(r: Raise<GuestError>)
    private fun evalTemplate(
        template: StringTemplate,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        val out = StringBuilder()

        fun step(
            index: Int,
            currentFail: Fail,
        ) {
            if (index == template.fragments.size) {
                ok(GValue.VString(out.toString()), currentFail)
                return
            }
            eval(template.fragments[index], env, { value, more ->
                out.append(sicp.guest.renderPrinted(value) ?: r.raise(GuestError.UnassignedRead(template.span)))
                step(index + 1, more)
            }, currentFail)
        }
        step(0, fail)
    }

    context(r: Raise<GuestError>)
    private fun evalBinary(
        expression: Binary,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        if (expression.operator == "&&") {
            eval(expression.left, env, { left, leftMore ->
                if (!Primitives.truth(left, expression.left.span)) {
                    ok(GValue.VBool(false), leftMore)
                } else {
                    eval(expression.right, env, { right, more ->
                        ok(GValue.VBool(Primitives.truth(right, expression.right.span)), more)
                    }, leftMore)
                }
            }, fail)
            return
        }
        if (expression.operator == "||") {
            eval(expression.left, env, { left, leftMore ->
                if (Primitives.truth(left, expression.left.span)) {
                    ok(GValue.VBool(true), leftMore)
                } else {
                    eval(expression.right, env, { right, more ->
                        ok(GValue.VBool(Primitives.truth(right, expression.right.span)), more)
                    }, leftMore)
                }
            }, fail)
            return
        }
        eval(expression.left, env, { left, leftMore ->
            eval(expression.right, env, { right, more ->
                ok(Primitives.binary(expression.operator, left, right, expression.span), more)
            }, leftMore)
        }, fail)
    }

    context(r: Raise<GuestError>)
    private fun evalIf(
        expression: If,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        eval(expression.condition, env, { condition, more ->
            val branch = if (Primitives.truth(condition, expression.condition.span)) expression.yes else expression.no
            if (branch != null) eval(branch, env, ok, more) else ok(GValue.VUnit, more)
        }, fail)
    }

    context(r: Raise<GuestError>)
    private fun evalWhen(
        expression: When,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        val subjectExpression = expression.subject
        if (subjectExpression == null) {
            evalWhenGuards(expression, env, ok, fail)
            return
        }
        eval(subjectExpression, env, { subject, _ ->
            var entered = false
            for (branch in expression.branches) {
                if (branchMatches(branch, subject, env)) {
                    entered = true
                    eval(branch.body, env, ok, fail)
                    break
                }
            }
            if (!entered) {
                run {
                    val otherwise = expression.otherwise
                    if (otherwise != null) eval(otherwise, env, ok, fail) else ok(GValue.VUnit, fail)
                }
            }
        }, fail)
    }

    context(r: Raise<GuestError>)
    private fun evalWhenGuards(
        expression: When,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        var entered = false
        for (branch in expression.branches) {
            val guard = branch.pattern ?: continue
            if (Primitives.truth(evalDirect(guard, env), guard.span)) {
                entered = true
                eval(branch.body, env, ok, fail)
                break
            }
        }
        if (!entered) {
            run {
                val otherwise = expression.otherwise
                if (otherwise != null) eval(otherwise, env, ok, fail) else ok(GValue.VUnit, fail)
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun branchMatches(
        branch: WhenBranch,
        subject: GValue?,
        env: Env,
    ): Boolean {
        val typePattern = branch.typePattern
        if (typePattern != null) return subject != null && Primitives.isTypeValue(subject, typePattern)
        val pattern = branch.pattern ?: return false
        if (subject == null) return Primitives.truth(evalDirect(pattern, env), pattern.span)
        return valueEquals(evalDirect(pattern, env), subject)
    }

    context(r: Raise<GuestError>)
    private fun evalDirect(
        expression: Expression,
        env: Env,
    ): GValue {
        var answer: GValue = GValue.VUnit
        synchronousDepth++
        try {
            eval(expression, env, { value, _ -> answer = value }, {})
        } finally {
            synchronousDepth--
        }
        return answer
    }

    context(r: Raise<GuestError>)
    private fun evalBlock(
        block: Block,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        if (block in checked.lambdaCoercions) {
            ok(lambdaValueOf(block.statements, emptyList(), env), fail)
            return
        }
        runStatements(block.statements, Env.child(env), ok, fail)
    }

    // ---------- calls, intrinsics, members ----------

    context(r: Raise<GuestError>)
    private fun evalCall(
        expression: Call,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        val callee = expression.callee
        if (callee is Name && callee.text in searchIntrinsics) {
            evalIntrinsic(callee.text, expression, env, ok, fail)
            return
        }
        if (callee is Name) {
            val bound = env.lookup(callee.text) ?: globals.lookup(callee.text)
            if (bound != null && bound.value !is GValue.VUnassigned) {
                evalArguments(expression, env, { arguments, more ->
                    applyFunction(bound.value, arguments, ok, more)
                }, fail)
                return
            }
            val shape = classTable[callee.text]
            if (shape != null) {
                evalArguments(expression, env, { arguments, more ->
                    val fields = linkedMapOf<String, GValue>()
                    for ((property, value) in shape.properties.zip(arguments)) fields[property.name] = value
                    ok(GValue.VObject(shape.name, structural = shape.data, fields), more)
                }, fail)
                return
            }
            evalArguments(expression, env, { arguments, more ->
                ok(Primitives.call(callee.text, arguments, sink, expression.span), more)
            }, fail)
            return
        }
        if (callee is Member) {
            eval(callee.receiver, env, { receiver, _ ->
                evalArguments(expression, env, { arguments, more ->
                    val shape = (receiver as? GValue.VObject)?.let { classTable[it.className] }
                    val method = shape?.methods?.get(callee.name)
                    if (method != null) {
                        applyFunction(functionValue(method, globals), arguments, ok, more)
                    } else {
                        memberCallCps(receiver, callee.name, arguments, ok, more, expression.span)
                    }
                }, fail)
            }, fail)
            return
        }
        eval(callee, env, { fn, _ ->
            evalArguments(expression, env, { arguments, more -> applyFunction(fn, arguments, ok, more) }, fail)
        }, fail)
    }

    private val searchIntrinsics: Set<String> = setOf("choose", "chooseRandom", "demand", "setPermanent", "ifFail", "seededRandom")

    context(r: Raise<GuestError>)
    private fun evalIntrinsic(
        name: String,
        expression: Call,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        when (name) {
            "choose" -> evalChoose(expression, env, ok, fail, randomized = false)
            "chooseRandom" -> evalChoose(expression, env, ok, fail, randomized = true)
            "demand" -> evalDemand(expression, env, ok, fail)
            "setPermanent" -> evalSetPermanent(expression, env, ok, fail)
            "ifFail" -> evalIfFail(expression, env, ok, fail)
            else -> evalSeededRandom(expression, env, ok, fail)
        }
    }

    context(r: Raise<GuestError>)
    private fun evalChoose(
        expression: Call,
        env: Env,
        ok: Ok,
        fail: Fail,
        randomized: Boolean,
    ) {
        val alternatives = expression.arguments.map { it.value }
        if (alternatives.isEmpty()) {
            fail()
            return
        }
        val order =
            if (randomized) {
                val generator = random ?: GValue.VRandom(0L).also { random = it }
                shuffledIndices(alternatives.size, generator)
            } else {
                alternatives.indices.toList()
            }
        val snapshot = snapshotCells(env)
        val mark = mutations.size

        fun tryAt(position: Int) {
            if (position == order.size) {
                fail()
                return
            }
            val limit = choiceLimit
            if (limit != null && state.choices >= limit) {
                choiceHorizonReached = true
                throw ChoiceHorizon()
            }
            restoreMutated(mark)
            restoreCells(snapshot)
            replayPermanent()
            state.choices++
            eval(alternatives[order[position]], env, ok, {
                enqueue { tryAt(position + 1) }
            })
        }
        if (synchronousDepth > 0) tryAt(0) else enqueue { tryAt(0) }
    }

    private fun shuffledIndices(
        size: Int,
        generator: GValue.VRandom,
    ): List<Int> {
        val out = (0 until size).toMutableList()
        for (i in size - 1 downTo 1) {
            generator.seed = generator.seed * 6364136223846793005L + 1442695040888963407L
            val pick = ((generator.seed ushr 16) % (i + 1)).toInt()
            val swap = out[i]
            out[i] = out[pick]
            out[pick] = swap
        }
        return out
    }

    context(r: Raise<GuestError>)
    private fun evalDemand(
        expression: Call,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        val condition = expression.arguments.single().value
        eval(condition, env, { value, more ->
            if (Primitives.truth(value, condition.span)) ok(GValue.VUnit, more) else more()
        }, fail)
    }

    context(r: Raise<GuestError>)
    private fun evalSetPermanent(
        expression: Call,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        val body = expression.arguments.single().value
        eval(body, env, { procedure, afterBody ->
            val outside = permanentDepth
            permanentDepth = outside + 1
            try {
                applyFunction(
                    procedure,
                    emptyList(),
                    { value, more ->
                        val previous = permanentDepth
                        permanentDepth = outside
                        try {
                            ok(value, {
                                val resumeDepth = permanentDepth
                                permanentDepth = outside + 1
                                try {
                                    more()
                                } finally {
                                    permanentDepth = resumeDepth
                                }
                            })
                        } finally {
                            permanentDepth = previous
                        }
                    },
                    {
                        val previous = permanentDepth
                        permanentDepth = outside
                        try {
                            afterBody()
                        } finally {
                            permanentDepth = previous
                        }
                    },
                )
            } finally {
                permanentDepth = outside
            }
        }, fail)
    }

    context(r: Raise<GuestError>)
    private fun evalIfFail(
        expression: Call,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        val primary = expression.arguments[0].value
        val fallback = expression.arguments[1].value
        eval(primary, env, { procedure, _ ->
            applyFunction(procedure, emptyList(), ok, {
                eval(fallback, env, { alternative, more ->
                    applyFunction(alternative, emptyList(), ok, more)
                }, fail)
            })
        }, fail)
    }

    context(r: Raise<GuestError>)
    private fun evalSeededRandom(
        expression: Call,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        val seedExpression = expression.arguments.single().value
        eval(seedExpression, env, { value, more ->
            val seed = (value as? GValue.VLong)?.value ?: r.raise(GuestError.UnassignedRead(expression.span))
            val generator = GValue.VRandom(seed)
            random = generator
            ok(generator, more)
        }, fail)
    }

    context(r: Raise<GuestError>)
    private fun evalArguments(
        expression: Call,
        env: Env,
        ok: (List<GValue>, Fail) -> Unit,
        fail: Fail,
    ) {
        fun step(
            index: Int,
            prefix: List<GValue>,
            currentFail: Fail,
        ) {
            if (index == expression.arguments.size) {
                ok(prefix, currentFail)
                return
            }
            eval(expression.arguments[index].value, env, { value, more ->
                step(index + 1, prefix + value, more)
            }, currentFail)
        }
        step(0, emptyList(), fail)
    }

    context(r: Raise<GuestError>)
    private fun applyFunction(
        fn: GValue,
        arguments: List<GValue>,
        ok: Ok,
        fail: Fail,
    ) {
        val closure = closures[fn]
        if (closure != null) {
            runClosureStreaming(closure, arguments, ok, fail)
            return
        }
        if (fn !is GValue.VFunction) {
            ok(Primitives.invoke(fn, arguments, NO_POSITION), fail)
            return
        }
        ok(fn.apply(r, arguments), fail)
    }

    context(r: Raise<GuestError>)
    private fun runClosureStreaming(
        shape: ClosureShape,
        arguments: List<GValue>,
        ok: Ok,
        fail: Fail,
    ) {
        val env = Env.child(shape.definingEnv)
        val deliver: Ok = if (synchronousDepth > 0) ok else { value, more -> enqueue { ok(value, more) } }
        returnContinuations[env] = deliver
        for ((name, value) in shape.parameters.zip(arguments)) env.define(name, value)
        val statements = shape.statements
        if (statements == null) {
            eval(shape.expression ?: return, env, deliver, fail)
            return
        }
        runStatements(statements, env, deliver, fail)
    }

    context(r: Raise<GuestError>)
    private fun evalMember(
        expression: Member,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        eval(expression.receiver, env, { receiver, more ->
            if (receiver is GValue.VObject) {
                val field = receiver.fields[expression.name]
                if (field != null) {
                    ok(field, more)
                    return@eval
                }
                val method = classTable[receiver.className]?.methods?.get(expression.name)
                if (method != null) {
                    ok(functionValue(method, globals), more)
                    return@eval
                }
            }
            ok(Primitives.property(receiver, expression.name, expression.span), more)
        }, fail)
    }

    // ---------- statements ----------

    context(r: Raise<GuestError>)
    private fun execStatement(
        statement: Statement,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        when (statement) {
            is ExpressionStatement -> {
                eval(statement.expression, env, ok, fail)
            }

            is LocalProperty -> {
                eval(statement.initializer, env, { value, more ->
                    env.define(statement.name, value)
                    ok(GValue.VUnit, more)
                }, fail)
            }

            is Destructure -> {
                eval(statement.initializer, env, { source, more ->
                    val values =
                        when (source) {
                            is GValue.VPair -> listOf(source.first, source.second)
                            is GValue.VObject -> source.fields.values.toList()
                            else -> r.raise(GuestError.UnassignedRead(statement.span))
                        }
                    for ((name, value) in statement.names.zip(values)) env.define(name, value)
                    ok(GValue.VUnit, more)
                }, fail)
            }

            is Assignment -> {
                eval(statement.value, env, { value, more ->
                    writeAssignment(statement, value, env)
                    ok(GValue.VUnit, more)
                }, fail)
            }

            is While -> {
                runWhile(statement, env, ok, fail)
            }

            is For -> {
                runFor(statement, env, ok, fail)
            }

            is Return -> {
                val returnOk = returnTo(env) ?: r.raise(GuestError.ShapeFault(statement.span))
                val operand = statement.value
                if (operand == null) {
                    returnOk(GValue.VUnit, fail)
                } else {
                    eval(operand, env, { value, more -> returnOk(value, more) }, fail)
                }
            }

            is Break -> {
                throw BreakSignal()
            }

            is Continue -> {
                throw ContinueSignal()
            }

            is FunctionDecl -> {
                env.define(statement.name, functionValue(statement, env))
                ok(GValue.VUnit, fail)
            }
        }
    }

    context(r: Raise<GuestError>)
    private fun writeAssignment(
        statement: Assignment,
        value: GValue,
        env: Env,
    ) {
        val target = statement.target
        if (target is Name) {
            val cell = env.lookup(target.text) ?: globals.lookup(target.text) ?: r.raise(GuestError.UnassignedRead(target.span))
            if (permanentDepth > 0) permanent[cell] = value
            cell.value = value
            return
        }
        if (target is Member) {
            val receiver = evalDirect(target.receiver, env) as? GValue.VObject ?: r.raise(GuestError.UnassignedRead(target.span))
            val before = FieldsState(receiver, LinkedHashMap(receiver.fields))
            receiver.fields[target.name] = value
            recordWrite(before, FieldsState(receiver, LinkedHashMap(receiver.fields)))
            return
        }
        if (target is Index) {
            val receiver = evalDirect(target.receiver, env)
            val index = evalDirect(target.index, env)
            val before = storeState(receiver, statement.span)
            Primitives.writeIndex(receiver, index, value, statement.span)
            recordWrite(before, storeState(receiver, statement.span))
            return
        }
        r.raise(GuestError.UnassignedRead(statement.span))
    }

    context(r: Raise<GuestError>)
    private fun runWhile(
        statement: While,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        eval(statement.condition, env, { condition, _ ->
            if (!Primitives.truth(condition, statement.condition.span)) {
                ok(GValue.VUnit, fail)
            } else {
                try {
                    runStatements(statement.body.statements, Env.child(env), { _, more -> runWhile(statement, env, ok, more) }, fail)
                } catch (_: BreakSignal) {
                    ok(GValue.VUnit, fail)
                } catch (_: ContinueSignal) {
                    runWhile(statement, env, ok, fail)
                }
            }
        }, fail)
    }

    context(r: Raise<GuestError>)
    private fun runFor(
        statement: For,
        env: Env,
        ok: Ok,
        fail: Fail,
    ) {
        val loopEnv = Env.child(env)
        val items = forItems(statement, env)
        var index = 0

        fun step(currentFail: Fail) {
            if (index == items.size) {
                ok(GValue.VUnit, currentFail)
                return
            }
            loopEnv.define(statement.name, items[index])
            index++
            try {
                runStatements(statement.body.statements, Env.child(loopEnv), { _, more -> step(more) }, currentFail)
            } catch (_: BreakSignal) {
                ok(GValue.VUnit, currentFail)
            } catch (_: ContinueSignal) {
                step(currentFail)
            }
        }
        step(fail)
    }

    context(r: Raise<GuestError>)
    private fun forItems(
        statement: For,
        env: Env,
    ): List<GValue> {
        val endExpression = statement.end
        if (endExpression != null) {
            val start = evalDirect(statement.iterable, env)
            val end = evalDirect(endExpression, env)
            return when {
                start is GValue.VInt && end is GValue.VInt -> (start.value..end.value).map { GValue.VInt(it) }
                start is GValue.VLong && end is GValue.VLong -> (start.value..end.value).map { GValue.VLong(it) }
                else -> r.raise(GuestError.UnassignedRead(statement.span))
            }
        }
        return when (val source = evalDirect(statement.iterable, env)) {
            is GValue.VList -> source.items.toList()
            is GValue.VLazyList -> (sicp.guest.materialize(source) as GValue.VList).items.toList()
            else -> r.raise(GuestError.UnassignedRead(statement.span))
        }
    }

    private fun snapshotCells(env: Env): List<Pair<Cell, GValue>> {
        val out = mutableListOf<Pair<Cell, GValue>>()
        var here: Env? = env
        while (here != null) {
            for (cell in here.bindings.values) out.add(cell to cell.value)
            here = here.parent
        }
        return out
    }

    private fun restoreCells(snapshot: List<Pair<Cell, GValue>>) {
        for ((cell, value) in snapshot) cell.value = value
    }

    private fun replayPermanent() {
        for ((cell, value) in permanent) cell.value = value
        for (store in permanentMutations) store.restore()
    }

    private fun restoreMutated(mark: Int) {
        for (index in mutations.lastIndex downTo mark) mutations[index].restore()
        while (mutations.size > mark) mutations.removeAt(mutations.size - 1)
    }

    private fun recordWrite(
        before: StoreState,
        after: StoreState,
    ) {
        if (permanentDepth > 0) {
            permanentMutations.add(after)
        } else {
            mutations.add(before)
        }
    }

    context(r: Raise<GuestError>)
    private fun storeState(
        store: GValue,
        at: Span,
    ): StoreState =
        when (store) {
            is GValue.VList -> ListState(store, store.items.toMutableList())
            is GValue.VMap -> MapState(store, LinkedHashMap(store.entries))
            is GValue.VObject -> FieldsState(store, LinkedHashMap(store.fields))
            else -> r.raise(GuestError.ShapeFault(at))
        }

    /** Collection and map members whose callback runs guest code: the
     * search evaluator iterates them through its continuations so choices
     * inside the callback multiply like any other choice. */
    context(r: Raise<GuestError>)
    private fun memberCallCps(
        receiver: GValue,
        name: String,
        arguments: List<GValue>,
        ok: Ok,
        more: Fail,
        at: Span,
    ) {
        val mutating = name == "add" || name == "set" || name == "put" || name == "remove"
        if (mutating && (receiver is GValue.VList || receiver is GValue.VMap)) {
            val before = storeState(receiver, at)
            val result = Primitives.member(receiver, name, arguments, at)
            recordWrite(before, storeState(receiver, at))
            ok(result, more)
            return
        }
        val callbackIndex = if (name == "fold") 1 else 0
        val callback = arguments.getOrNull(callbackIndex)
        val items = collectionItems(receiver)
        if (callback == null || items == null || name !in CALLBACK_MEMBERS) {
            ok(Primitives.member(receiver, name, arguments, at), more)
            return
        }
        when (name) {
            "map" -> callbackMap(items, callback, emptyList(), ok, more, at)
            "filter" -> callbackFilter(items, callback, emptyList(), ok, more, at)
            "fold" -> callbackFold(items, callback, arguments[0], ok, more, at)
            "any" -> callbackAny(items, callback, ok, more, at)
            else -> callbackAll(items, callback, ok, more, at)
        }
    }

    context(r: Raise<GuestError>)
    private fun collectionItems(receiver: GValue): List<GValue>? {
        val source = if (receiver is GValue.VLazyList) sicp.guest.materialize(receiver) else receiver
        if (source is GValue.VList) return source.items.toList()
        return null
    }

    context(r: Raise<GuestError>)
    private fun callbackMap(
        items: List<GValue>,
        fn: GValue,
        acc: List<GValue>,
        ok: Ok,
        more: Fail,
        at: Span,
    ) {
        if (items.isEmpty()) {
            ok(GValue.VList(acc.toMutableList(), mutable = false), more)
            return
        }
        applyFunction(fn, listOf(items.first()), { value, next -> callbackMap(items.drop(1), fn, acc + value, ok, next, at) }, more)
    }

    context(r: Raise<GuestError>)
    private fun callbackFilter(
        items: List<GValue>,
        pred: GValue,
        acc: List<GValue>,
        ok: Ok,
        more: Fail,
        at: Span,
    ) {
        if (items.isEmpty()) {
            ok(GValue.VList(acc.toMutableList(), mutable = false), more)
            return
        }
        val item = items.first()
        applyFunction(pred, listOf(item), { answer, next ->
            if (Primitives.truth(answer, at)) {
                callbackFilter(items.drop(1), pred, acc + item, ok, next, at)
            } else {
                callbackFilter(items.drop(1), pred, acc, ok, next, at)
            }
        }, more)
    }

    context(r: Raise<GuestError>)
    private fun callbackFold(
        items: List<GValue>,
        fn: GValue,
        acc: GValue,
        ok: Ok,
        more: Fail,
        at: Span,
    ) {
        if (items.isEmpty()) {
            ok(acc, more)
            return
        }
        applyFunction(fn, listOf(acc, items.first()), { value, next -> callbackFold(items.drop(1), fn, value, ok, next, at) }, more)
    }

    context(r: Raise<GuestError>)
    private fun callbackAny(
        items: List<GValue>,
        pred: GValue,
        ok: Ok,
        more: Fail,
        at: Span,
    ) {
        if (items.isEmpty()) {
            ok(GValue.VBool(false), more)
            return
        }
        applyFunction(pred, listOf(items.first()), { answer, next ->
            if (Primitives.truth(answer, at)) {
                ok(GValue.VBool(true), next)
            } else {
                callbackAny(items.drop(1), pred, ok, next, at)
            }
        }, more)
    }

    context(r: Raise<GuestError>)
    private fun callbackAll(
        items: List<GValue>,
        pred: GValue,
        ok: Ok,
        more: Fail,
        at: Span,
    ) {
        if (items.isEmpty()) {
            ok(GValue.VBool(true), more)
            return
        }
        applyFunction(pred, listOf(items.first()), { answer, next ->
            if (Primitives.truth(answer, at)) {
                callbackAll(items.drop(1), pred, ok, next, at)
            } else {
                ok(GValue.VBool(false), next)
            }
        }, more)
    }
}

/** A mutable store captured whole: restoring the snapshot in reverse
 * rewinds every write the search made past a choice point, including the
 * entry order of maps and the order of list items. */
private sealed interface StoreState {
    fun restore()
}

private class FieldsState(
    val obj: GValue.VObject,
    val fields: Map<String, GValue>,
) : StoreState {
    override fun restore() {
        obj.fields.clear()
        obj.fields.putAll(fields)
    }
}

private class ListState(
    val list: GValue.VList,
    val items: List<GValue>,
) : StoreState {
    override fun restore() {
        list.items.clear()
        list.items.addAll(items)
    }
}

private class MapState(
    val map: GValue.VMap,
    val entries: Map<GValue, GValue>,
) : StoreState {
    override fun restore() {
        map.entries.clear()
        map.entries.putAll(entries)
    }
}

private val CALLBACK_MEMBERS: Set<String> = setOf("map", "filter", "fold", "any", "all")
