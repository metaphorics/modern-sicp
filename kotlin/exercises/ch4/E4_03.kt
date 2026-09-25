// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.3

package sicp.ch4.exercises

import arrow.core.raise.Raise
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.Key
import sicp.runtime.PendingSolution
import sicp.runtime.SchemeError
import sicp.runtime.Value

/** The ordered tag-list key of D19: `["a", "b"]` becomes the chain `(a b)`. */
public fun evalTagKey(tags: List<String>): Key = tags.foldRight(Key.Nil as Key) { t, acc -> Key.Pair(Key.Sym(t), acc) }

/**
 * One eval clause: the form, the dispatcher to recurse through, and the
 * environment. The runtime `OpTable` stores `Op = List<Value> -> Value`
 * handlers, a shape that cannot carry a form and an environment, so this
 * table keeps the put/get registry semantics of D19 -- absent option on a
 * miss, overwrite on put -- over this clause type.
 */
public typealias EvalClause = context(Raise<SchemeError>)
(TableDriven, Expr, Env) -> Value

/**
 * Exercise 4.3: the data-directed eval. Every compound clause lives in the
 * table under `(eval, tag)`, installed at startup; handlers recurse through
 * the evaluator, so a later `put` redirects the whole evaluator at every
 * depth. Self-evaluating expressions and variables stay residual tests
 * (the 2.73 lesson), a form with no entry falls back to the base clause
 * chain, and `(cond ((= 1 2) 'no) (else 'yes))` answers yes through the
 * table.
 */
public class TableDriven(
    global: Env,
) : Evaluator(global) {
    /** The book's `put`: installs [clause] under `(op, tag)`, overwriting. */
    public fun put(
        op: Key,
        tag: Key,
        clause: EvalClause,
    ): Unit = throw PendingSolution()

    /** The book's `get`: the clause under `(op, tag)`, or null. */
    public fun get(
        op: Key,
        tag: Key,
    ): EvalClause? = throw PendingSolution()

    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep = throw PendingSolution()
}
