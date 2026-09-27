// SPDX-License-Identifier: GPL-3.0-only
// The metacircular evaluator of sections 4.1.1 to 4.1.5. `Evaluator` is the
// book's `eval`/`apply` case analysis with one extension seam: `step` is the
// clause chain, and every exercise that edits the evaluator subclasses it,
// checks its forms in `step`, and falls back to `baseStep`, so the extension
// fires at every nesting depth. Plain base behavior is one call: `eval`.
//
// Tail calls: Kotlin has no mutual tail-call elimination (the edition's
// sketch 5), so `eval` is a dispatch loop that unwinds the book's tail
// positions -- if branches, the last expression of a sequence, and the body
// of an applied compound procedure -- in constant host stack, like on a
// Scheme host. That covers the corpus's 100,000-deep `count-down`; deep
// NON-tail recursion still costs host frames, the corpus keeps such
// programs small, and the explicit-control evaluator of 5.4 is the real
// answer, which the chapter itself points to.

package sicp.ch4

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.toPersistentList
import sicp.runtime.AppE
import sicp.runtime.BeginE
import sicp.runtime.CondClause
import sicp.runtime.CondE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.IfE
import sicp.runtime.LambdaE
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.QuoteE
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.VPrimitive
import sicp.runtime.VProc
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.VarE
import sicp.runtime.callPrimitive
import sicp.runtime.isTrue
import sicp.runtime.listItems
import sicp.runtime.vlist

/** One round of the dispatch loop: a value, or the next tail expression. */
public sealed interface EvalStep {
    /** The evaluation produced a value. */
    public data class Done(
        val v: Value,
    ) : EvalStep

    /** The evaluation continues with this expression in this environment,
     * the book's tail position made explicit. */
    public data class Continue(
        val expr: Expr,
        val env: Env,
    ) : EvalStep
}

/** The book's `cond->if` over typed clauses: `expand-clauses`. */
public fun condToIfChain(clauses: PersistentList<CondClause>): Expr {
    if (clauses.isEmpty()) return VarE("false") // no else clause
    val first = clauses.first()
    val rest = clauses.removingAt(0)
    val alternative = condToIfChain(rest)
    return when (first) {
        is CondClause.Else -> sequenceToExp(first.body)
        is CondClause.Clause -> IfE(first.test, sequenceToExp(first.body), alternative)
    }
}

/** The book's `sequence->exp`: one expression, `begin` when necessary. */
internal fun sequenceToExp(actions: PersistentList<Expr>): Expr = if (actions.size == 1) actions.first() else BeginE(actions)

/** `let` as a derived expression: the lambda-application combination. */
public fun letToCombinationExpr(expr: LetE): Expr {
    val lambda = LambdaE(expr.bindings.map { it.name }.toPersistentList(), null, expr.body)
    return AppE(lambda, expr.bindings.map { it.value }.toPersistentList())
}

/** Raises [error] through the ambient [Raise]; the bridge Exec bodies use,
 * where the context value has no name. */
context(r: Raise<SchemeError>)
public fun fail(error: SchemeError): Nothing = r.raise(error)

/**
 * The base evaluator. The book's clause chain lives in [step] and its
 * `baseStep` fallback; applications recurse through [eval] so an overridden
 * `step` sees every subexpression, and the compound-procedure body hops the
 * loop instead of deepening the host stack.
 */
public open class Evaluator(
    /** The global environment the driver set up. */
    public val global: Env,
) {
    /** Evaluates `expr` in `env`; the book's `eval` entry point. */
    context(r: Raise<SchemeError>)
    public fun eval(
        expr: Expr,
        env: Env,
    ): Value {
        var e = expr
        var en = env
        while (true) {
            when (val s = step(e, en)) {
                is EvalStep.Done -> {
                    return s.v
                }

                is EvalStep.Continue -> {
                    e = s.expr
                    en = s.env
                }
            }
        }
    }

    /** The extension seam: check the forms you add, then fall back to
     * [baseStep]. Fires at every nesting depth because every clause
     * recurses through [eval]. */
    context(r: Raise<SchemeError>)
    public open fun step(
        expr: Expr,
        env: Env,
    ): EvalStep = baseStep(expr, env)

    /** The book's clause chain of 4.1.1. */
    context(r: Raise<SchemeError>)
    protected fun baseStep(
        expr: Expr,
        env: Env,
    ): EvalStep =
        when (expr) {
            is LitE -> EvalStep.Done(expr.v)

            // self-evaluating?
            is VarE -> EvalStep.Done(lookupVariable(expr.name, env))

            // variable?
            is QuoteE -> EvalStep.Done(expr.datum)

            // quoted?
            is SetE -> evalAssignment(expr, env)

            // assignment?
            is DefineE -> evalDefinition(expr, env)

            // definition?
            is IfE -> evalIf(expr, env)

            // if?
            is LambdaE -> EvalStep.Done(makeProcedure(expr.params, expr.rest, expr.body, env, null))

            // lambda?
            is BeginE -> evalSequence(expr.actions, env)

            // begin?
            is CondE -> EvalStep.Continue(condToIf(expr), env)

            // cond? -> derived
            is LetE -> EvalStep.Continue(letToCombination(expr), env)

            // derived
            is AppE -> evalApplication(expr, env) // application?
        }

    /** `(set! name value)`: install the nearest binding, answer `ok`. */
    context(r: Raise<SchemeError>)
    private fun evalAssignment(
        expr: SetE,
        env: Env,
    ): EvalStep {
        setVariable(expr.name, eval(expr.value, env), env)
        return EvalStep.Done(VSym("ok"))
    }

    /** Both definition shapes; a procedure definition names its [VProc]. */
    context(r: Raise<SchemeError>)
    private fun evalDefinition(
        expr: DefineE,
        env: Env,
    ): EvalStep {
        val value = expr.value
        val v =
            if (value is LambdaE) {
                makeProcedure(value.params, value.rest, value.body, env, expr.name)
            } else {
                eval(value, env)
            }
        defineVariable(expr.name, v, env)
        return EvalStep.Done(VSym("ok"))
    }

    /** Evaluates the predicate, then continues in the chosen branch. */
    context(r: Raise<SchemeError>)
    private fun evalIf(
        expr: IfE,
        env: Env,
    ): EvalStep {
        val predicate = eval(expr.predicate, env)
        return EvalStep.Continue(if (isTrue(predicate)) expr.consequent else expr.alternative, env)
    }

    /** Evaluates all but the last expression, continues with the last. */
    context(r: Raise<SchemeError>)
    private fun evalSequence(
        actions: PersistentList<Expr>,
        env: Env,
    ): EvalStep {
        if (actions.isEmpty()) r.raise(SchemeError.Parse("empty sequence"))
        for (i in 0 until actions.size - 1) eval(actions[i], env)
        return EvalStep.Continue(actions.last(), env)
    }

    /** The book's `list-of-values`; this host evaluates operands left to
     * right, the fixed order the reworded Exercise 4.1 names. */
    context(r: Raise<SchemeError>)
    protected open fun listOfValues(
        operands: PersistentList<Expr>,
        env: Env,
    ): List<Value> = operands.map { eval(it, env) }

    /** Operator and operands, then the application. A compound procedure
     * extends its captured environment, evaluates all but the last body
     * expression, and hops the loop with the last one. */
    context(r: Raise<SchemeError>)
    protected open fun evalApplication(
        expr: AppE,
        env: Env,
    ): EvalStep {
        val procedure = eval(expr.operator, env)
        val arguments = listOfValues(expr.operands, env)
        return applyProcedure(procedure, arguments)
    }

    /** The book's `apply`: primitives apply through the runtime; compound
     * procedures evaluate their analyzed-at-call-time body. The `apply` and
     * `map` primitives must apply procedure values, so they re-enter here
     * instead of running as plain handlers. */
    context(r: Raise<SchemeError>)
    protected open fun applyProcedure(
        procedure: Value,
        arguments: List<Value>,
    ): EvalStep =
        when (procedure) {
            is VPrimitive -> {
                when (procedure.name) {
                    "apply" -> {
                        if (arguments.size != 2) {
                            r.raise(SchemeError.WrongArity("apply", "2", arguments.size))
                        }
                        applyProcedure(arguments[0], listItems(arguments[1]))
                    }

                    "map" -> {
                        if (arguments.size != 2) {
                            r.raise(SchemeError.WrongArity("map", "2", arguments.size))
                        }
                        val items = listItems(arguments[1])
                        EvalStep.Done(vlist(items.map { applyFully(arguments[0], listOf(it)) }))
                    }

                    else -> {
                        EvalStep.Done(callPrimitive(procedure, arguments))
                    }
                }
            }

            is VProc -> {
                val frame = extendEnvironment(procedure.params.toList(), arguments, procedure.env, procedure.rest)
                for (i in 0 until procedure.body.size - 1) eval(procedure.body[i], frame)
                EvalStep.Continue(procedure.body.last(), frame)
            }

            else -> {
                r.raise(SchemeError.NotApplicable(procedure))
            }
        }

    /** Applies and, when the application landed in a tail position, runs
     * the loop to a value; the shape `map` needs. */
    context(r: Raise<SchemeError>)
    private fun applyFully(
        procedure: Value,
        arguments: List<Value>,
    ): Value =
        when (val s = applyProcedure(procedure, arguments)) {
            is EvalStep.Done -> s.v
            is EvalStep.Continue -> eval(s.expr, s.env)
        }

    /** The book's `make-procedure`: parameters, body, captured environment. */
    protected open fun makeProcedure(
        params: PersistentList<String>,
        rest: String?,
        body: PersistentList<Expr>,
        env: Env,
        name: String?,
    ): Value = VProc(params, rest, body, env, name)

    // The four environment operations of 4.1.3, over the runtime's frames.
    // Exercises 4.11 to 4.13 override them with their own representations.

    context(r: Raise<SchemeError>)
    protected open fun lookupVariable(
        name: String,
        env: Env,
    ): Value = env.lookup(name)

    context(r: Raise<SchemeError>)
    protected open fun setVariable(
        name: String,
        value: Value,
        env: Env,
    ) {
        env.set(name, value)
    }

    protected open fun defineVariable(
        name: String,
        value: Value,
        env: Env,
    ) {
        env.define(name, value)
    }

    context(r: Raise<SchemeError>)
    protected open fun extendEnvironment(
        names: List<String>,
        values: List<Value>,
        parent: Env,
        rest: String?,
    ): Env = Env.extend(names, values, parent, rest)

    /** The book's `cond->if` over the parsed clauses. */
    protected open fun condToIf(cond: CondE): Expr = condToIfChain(cond.clauses)

    /** `let` as a derived expression: the lambda-application combination. */
    protected open fun letToCombination(expr: LetE): Expr = letToCombinationExpr(expr)
}
