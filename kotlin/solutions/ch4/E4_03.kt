// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.3

package sicp.ch4.solutions

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.ch4.condToIfChain
import sicp.ch4.letToCombinationExpr
import sicp.runtime.AppE
import sicp.runtime.BeginE
import sicp.runtime.CondE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.IfE
import sicp.runtime.Key
import sicp.runtime.LambdaE
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.QuoteE
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.VarE
import sicp.runtime.isTrue

/** The ordered tag-list key of D19: `["a", "b"]` becomes the chain `(a b)`. */
public fun evalTagKey(tags: List<String>): Key = tags.foldRight(Key.Nil as Key) { t, acc -> Key.Pair(Key.Sym(t), acc) }

/**
 * One eval clause: the form, the dispatcher to recurse through, and the
 * environment. The runtime `OpTable` stores `Op = List<Value> -> Value`
 * handlers, a shape that cannot carry a form and an environment, so this
 * table keeps the put/get registry semantics of D19 -- absent option on a
 * miss, overwrite on put -- over this clause type, the same substitution
 * the sibling editions record.
 */
public typealias EvalClause = context(Raise<SchemeError>)
(TableDriven, Expr, Env) -> Value

/**
 * Exercise 4.3: the data-directed eval. Every compound clause lives in the
 * table under `(eval, tag)`, installed at startup; the clause handlers
 * recurse through [TableDriven.eval], so a later `put` redirects the whole
 * evaluator at every depth. Self-evaluating expressions and variables stay
 * residual tests -- they dispatch on the datum, not a tag, the 2.73 lesson
 * -- and a form with no table entry falls back to the base clause chain.
 */
public class TableDriven(
    global: Env,
) : Evaluator(global) {
    private val clauses = HashMap<Key, HashMap<Key, EvalClause>>()

    /** The book's `put`: installs [clause] under `(op, tag)`, overwriting
     * an earlier install of exactly that pair. */
    public fun put(
        op: Key,
        tag: Key,
        clause: EvalClause,
    ) {
        clauses.getOrPut(op) { HashMap() }[tag] = clause
    }

    /** The book's `get`: the clause under `(op, tag)`, or null -- the
     * absent option, never a false-ish sentinel. */
    public fun get(
        op: Key,
        tag: Key,
    ): EvalClause? = clauses[op]?.get(tag)

    init {
        val eval = Key.Sym("eval")
        put(eval, evalTagKey(listOf("quote"))) { _, expr, _ ->
            (expr as QuoteE).datum
        }
        put(eval, evalTagKey(listOf("define"))) { _, expr, env ->
            defineClause(expr as DefineE, env)
        }
        put(eval, evalTagKey(listOf("set!"))) { _, expr, env ->
            assignmentClause(expr as SetE, env)
        }
        put(eval, evalTagKey(listOf("if"))) { _, expr, env ->
            ifClause(expr as IfE, env)
        }
        put(eval, evalTagKey(listOf("lambda"))) { _, expr, env ->
            val lambda = expr as LambdaE
            makeProcedure(lambda.params, lambda.rest, lambda.body, env, null)
        }
        put(eval, evalTagKey(listOf("begin"))) { _, expr, env ->
            sequenceClause((expr as BeginE).actions, env)
        }
        put(eval, evalTagKey(listOf("cond"))) { _, expr, env ->
            eval(condToIfChain((expr as CondE).clauses), env)
        }
        put(eval, evalTagKey(listOf("let"))) { _, expr, env ->
            eval(letToCombinationExpr(expr as LetE), env)
        }
        put(eval, evalTagKey(listOf("application"))) { _, expr, env ->
            applicationClause(expr as AppE, env)
        }
    }

    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep =
        when (expr) {
            // residual tests: the datum dispatches, not a tag (2.73)
            is LitE, is VarE -> {
                super.step(expr, env)
            }

            else -> {
                val clause =
                    get(Key.Sym("eval"), evalTagKey(listOf(tagOf(expr))))
                        ?: get(Key.Sym("eval"), evalTagKey(listOf("application")))
                if (clause == null) super.step(expr, env) else EvalStep.Done(clause(this, expr, env))
            }
        }

    /** The tag of one form: the head symbol of an application, else the
     * syntactic type the parser recognized. */
    private fun tagOf(expr: Expr): String =
        when (expr) {
            is AppE -> (expr.operator as? VarE)?.name ?: "application"
            is QuoteE -> "quote"
            is DefineE -> "define"
            is SetE -> "set!"
            is IfE -> "if"
            is LambdaE -> "lambda"
            is BeginE -> "begin"
            is CondE -> "cond"
            is LetE -> "let"
            else -> "application" // LitE and VarE never reach the table
        }

    context(r: Raise<SchemeError>)
    private fun defineClause(
        expr: DefineE,
        env: Env,
    ): Value {
        val value = expr.value
        val v =
            if (value is LambdaE) {
                makeProcedure(value.params, value.rest, value.body, env, expr.name)
            } else {
                eval(value, env)
            }
        defineVariable(expr.name, v, env)
        return VSym("ok")
    }

    context(r: Raise<SchemeError>)
    private fun assignmentClause(
        expr: SetE,
        env: Env,
    ): Value {
        setVariable(expr.name, eval(expr.value, env), env)
        return VSym("ok")
    }

    context(r: Raise<SchemeError>)
    private fun ifClause(
        expr: IfE,
        env: Env,
    ): Value = eval(if (isTrue(eval(expr.predicate, env))) expr.consequent else expr.alternative, env)

    context(r: Raise<SchemeError>)
    private fun sequenceClause(
        actions: PersistentList<Expr>,
        env: Env,
    ): Value {
        for (i in 0 until actions.size - 1) eval(actions[i], env)
        return eval(actions.last(), env)
    }

    context(r: Raise<SchemeError>)
    private fun applicationClause(
        expr: AppE,
        env: Env,
    ): Value =
        when (val s = applyProcedure(eval(expr.operator, env), listOfValues(expr.operands, env))) {
            is EvalStep.Done -> s.v
            is EvalStep.Continue -> eval(s.expr, s.env)
        }
}
