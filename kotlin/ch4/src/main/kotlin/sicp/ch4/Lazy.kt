// SPDX-License-Identifier: GPL-3.0-only
// The lazy evaluator of section 4.2: compound procedures are non-strict in
// every argument, primitives stay strict. The layer rides the 4.1 step seam
// the way the chapter's own extension exercises do: the application clause
// reroutes to `evalApplication` (the operator forced through `actualValue`,
// the operands passed on unevaluated) and the `if` clause forces its
// predicate; every other form falls back to the base chain. A thunk is the
// runtime's memoized-once `VThunk` cell -- `Delayed(expr, env)` fills once
// into `Forced(value)`, the book's `thunk`/`evaluated-thunk` pair whose
// rewrite drops the expression and environment -- and `VThunkNoMemo` is the
// unmemoized probe variant exercises 4.27 and 4.29 count with. The demand
// sites the book names force here: primitive arguments, an `if` predicate,
// an operator about to be applied, and the driver's answer. The two
// strict-primitive boundaries get one ruling each: `apply` re-binds every
// applied value as a fresh expression thunk so an applied lazy procedure
// stays lazy, and `map` forces each applied result because a strict
// primitive delivers actual values.
//
// Host truth: Kotlin has no tail-call elimination (the edition's sketch 5).
// `eval`'s dispatch loop still unwinds the book's tail positions -- if
// branches, the last expression of a sequence, and the body of an applied
// compound procedure -- in constant host stack, so an application landing
// in a demand chain keeps the loop, not the stack. Forcing a deep lazy
// chain (the `solve` integral demanded at element 1000) is non-tail
// recursion and costs host frames, one nest per delayed element; the
// sessions keep such demands at the thousand-element scale of the section's
// own examples, and the explicit-control evaluator of 5.4 is the real
// answer.

package sicp.ch4

import arrow.core.raise.Raise
import arrow.core.raise.either
import kotlinx.collections.immutable.PersistentList
import sicp.runtime.AppE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.IfE
import sicp.runtime.LitE
import sicp.runtime.SchemeError
import sicp.runtime.ThunkState
import sicp.runtime.VPrimitive
import sicp.runtime.VProc
import sicp.runtime.VThunk
import sicp.runtime.VThunkNoMemo
import sicp.runtime.Value
import sicp.runtime.callPrimitive
import sicp.runtime.isTrue
import sicp.runtime.listItems
import sicp.runtime.vlist

/**
 * The lazy evaluator. The book's rerouted clauses live in [step] (the `if`
 * that forces) and [evalApplication] (the application that delays); a thunk
 * forces at most once through [forceValue].
 */
public open class LazyEvaluator(
    /** The global environment the driver set up. */
    global: Env,
) : Evaluator(global) {
    /** The book's `actual-value`: `eval`, then force what came back. */
    context(r: Raise<SchemeError>)
    public fun actualValue(
        expr: Expr,
        env: Env,
    ): Value = forceValue(eval(expr, env))

    /** The book's `force-it`: a memoized [VThunk] forces once and stores
     * the fully forced answer (the book's `evaluated-thunk` rewrite); a
     * [VThunkNoMemo] re-runs the expression at every demand; anything else
     * is already a value. */
    context(r: Raise<SchemeError>)
    public open fun forceValue(v: Value): Value =
        when (v) {
            is VThunk -> {
                when (val state = v.state) {
                    is ThunkState.Forced -> state.v
                    is ThunkState.Delayed -> forceDelayed(v, state)
                }
            }

            is VThunkNoMemo -> {
                actualValue(v.expr, v.env)
            }

            else -> {
                v
            }
        }

    /** Fills a delayed cell once: the answer of forcing the stored
     * expression, memoized into the shared object. */
    context(r: Raise<SchemeError>)
    private fun forceDelayed(
        thunk: VThunk,
        state: ThunkState.Delayed,
    ): Value {
        val answer = actualValue(state.expr, state.env)
        thunk.state = ThunkState.Forced(answer)
        return answer
    }

    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep =
        when (expr) {
            // The one non-application demand site: the predicate's value
            // decides, so forcing it is not optional.
            is IfE -> forcedIf(expr, env)

            else -> super.step(expr, env)
        }

    /** The book's `eval-if`: the predicate forced, the chosen branch still
     * a tail. */
    context(r: Raise<SchemeError>)
    private fun forcedIf(
        expr: IfE,
        env: Env,
    ): EvalStep {
        val predicate = actualValue(expr.predicate, env)
        return EvalStep.Continue(if (isTrue(predicate)) expr.consequent else expr.alternative, env)
    }

    /** The application clause of 4.2.2: the operator forced, the operands
     * handed on unevaluated. */
    context(r: Raise<SchemeError>)
    override fun evalApplication(
        expr: AppE,
        env: Env,
    ): EvalStep {
        val procedure = actualValue(expr.operator, env)
        return applyDelaying(procedure, expr.operands, env)
    }

    /** The book's new `apply`, over unevaluated operand expressions:
     * primitives (strict) evaluate every operand first; compound procedures
     * (non-strict) bind thunks and hop the loop with the last body
     * expression after [evalBodyPrefix] runs the rest. */
    context(r: Raise<SchemeError>)
    protected open fun applyDelaying(
        procedure: Value,
        operands: PersistentList<Expr>,
        env: Env,
    ): EvalStep =
        when (procedure) {
            is VPrimitive -> applyPrimitive(procedure, operands, env)
            is VProc -> applyCompound(procedure, delayOperands(operands, env), env)
            else -> r.raise(SchemeError.NotApplicable(procedure))
        }

    /** The strict path: every operand is forced, because a primitive will
     * use its value. The intercepted `apply` and `map` names re-enter the
     * lazy discipline through [applyValue]. */
    context(r: Raise<SchemeError>)
    private fun applyPrimitive(
        procedure: VPrimitive,
        operands: PersistentList<Expr>,
        env: Env,
    ): EvalStep =
        when (procedure.name) {
            "apply" -> {
                val arguments = listOfArgValues(operands, env)
                checkArity("apply", arguments)
                applyValue(arguments[0], listItems(arguments[1]), env)
            }

            "map" -> {
                val arguments = listOfArgValues(operands, env)
                checkArity("map", arguments)
                val items = listItems(arguments[1])
                EvalStep.Done(vlist(items.map { applyAndForce(arguments[0], listOf(it), env) }))
            }

            else -> {
                EvalStep.Done(callPrimitive(procedure, listOfArgValues(operands, env)))
            }
        }

    /** An applied compound procedure: the delayed arguments extend the
     * captured environment, the body prefix runs, the last expression stays
     * a tail. */
    context(r: Raise<SchemeError>)
    private fun applyCompound(
        procedure: VProc,
        arguments: List<Value>,
        env: Env,
    ): EvalStep {
        val frame = extendEnvironment(procedure.params.toList(), arguments, procedure.env, procedure.rest)
        evalBodyPrefix(procedure.body, frame)
        return EvalStep.Continue(procedure.body.last(), frame)
    }

    /** The book's `apply` over already-evaluated arguments: a compound
     * procedure binds each value as a fresh expression thunk, so an applied
     * lazy procedure stays lazy (exercise 4.26's `(apply unless ...)`); a
     * primitive runs strictly. */
    context(r: Raise<SchemeError>)
    protected open fun applyValue(
        procedure: Value,
        values: List<Value>,
        env: Env,
    ): EvalStep =
        when (procedure) {
            is VPrimitive -> {
                when (procedure.name) {
                    "apply" -> {
                        checkArity("apply", values)
                        applyValue(values[0], listItems(values[1]), env)
                    }

                    "map" -> {
                        checkArity("map", values)
                        val items = listItems(values[1])
                        EvalStep.Done(vlist(items.map { applyAndForce(values[0], listOf(it), env) }))
                    }

                    else -> {
                        EvalStep.Done(callPrimitive(procedure, values))
                    }
                }
            }

            is VProc -> {
                val delayed = values.map { VThunk(ThunkState.Delayed(LitE(it), env)) }
                applyCompound(procedure, delayed, env)
            }

            else -> {
                r.raise(SchemeError.NotApplicable(procedure))
            }
        }

    /** Runs one application to a fully forced value; the shape `map`
     * needs, since a strict primitive's result is an actual value. */
    context(r: Raise<SchemeError>)
    private fun applyAndForce(
        procedure: Value,
        values: List<Value>,
        env: Env,
    ): Value {
        val answer =
            when (val s = applyValue(procedure, values, env)) {
                is EvalStep.Done -> s.v
                is EvalStep.Continue -> eval(s.expr, s.env)
            }
        return forceValue(answer)
    }

    context(r: Raise<SchemeError>)
    private fun checkArity(
        name: String,
        values: List<Value>,
    ) {
        if (values.size != 2) r.raise(SchemeError.WrongArity(name, "2", values.size))
    }

    /** The book's `list-of-arg-values`: the strict path demands actual
     * values. */
    context(r: Raise<SchemeError>)
    protected fun listOfArgValues(
        operands: PersistentList<Expr>,
        env: Env,
    ): List<Value> = operands.map { actualValue(it, env) }

    /** The book's `list-of-delayed-args`: the non-strict path packages each
     * operand expression with the environment of the application. The
     * memoization toggle of exercise 4.29 lives here. */
    protected open fun delayOperands(
        operands: PersistentList<Expr>,
        env: Env,
    ): List<Value> = operands.map { VThunk(ThunkState.Delayed(it, env)) }

    /** The body prefix of an applied compound procedure: the book's
     * `eval-sequence` over all but the last expression. The text's rule
     * evaluates without forcing; exercise 4.30 debates forcing here. */
    context(r: Raise<SchemeError>)
    protected open fun evalBodyPrefix(
        body: PersistentList<Expr>,
        env: Env,
    ) {
        for (i in 0 until body.size - 1) eval(body[i], env)
    }
}

/** The core of the lazy driver loop: evaluates [text]'s forms in order on
 * [evaluator], forcing every top-level answer before it prints, and returns
 * the printer-contract transcript -- defines print nothing, `display` and
 * `newline` print their side effect with no value line, one `Error:` line
 * stops the program. The book's `;;; L-Eval input:`/`;;; L-Eval value:`
 * prompts announce the REPL round-trips this fixed-program driver folds
 * away. */
public fun lazyTranscript(
    evaluator: LazyEvaluator,
    env: Env,
    sink: OutputSink,
    text: String,
): String {
    either {
        for (expr in parseProgram(readProgram(text))) {
            when {
                expr is DefineE -> evaluator.eval(expr, env)

                // a define prints nothing
                expr is AppE && isSinkCall(expr) -> evaluator.eval(expr, env)

                // display/newline: side effect only
                else -> sink.line(printValue(evaluator.actualValue(expr, env)))
            }
        }
    }.fold(
        { e -> sink.line("Error: ${formatError(e)}") },
        { },
    )
    return sink.toString()
}

/** Runs [text] as a lazy program on [env], recording into [sink]. */
public fun runLazyProgram(
    text: String,
    env: Env,
    sink: OutputSink,
): String = lazyTranscript(LazyEvaluator(env), env, sink, text)

/** Text in, transcript out, on a fresh lazy global environment. */
public fun runLazyProgram(text: String): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    return lazyTranscript(LazyEvaluator(env), env, sink, text)
}

/** Runs [text] on a factory-built lazy evaluator -- the variant evaluators
 * of the exercises (4.26, 4.29, 4.31, 4.33) pin their transcripts through
 * it. */
public fun lazyTranscriptOn(
    evaluatorFactory: (Env) -> LazyEvaluator,
    text: String,
    install: ((Env) -> Unit)? = null,
): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    install?.invoke(env)
    return lazyTranscript(evaluatorFactory(env), env, sink, text)
}
