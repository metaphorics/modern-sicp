// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.5

package sicp.ch4.solutions

import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import sicp.ch4.Evaluator
import sicp.runtime.AppE
import sicp.runtime.BeginE
import sicp.runtime.CondClause
import sicp.runtime.CondE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.IfE
import sicp.runtime.LambdaE
import sicp.runtime.VarE

/**
 * Exercise 4.5: cond clauses of the shape `(test => recipient)`. Taking
 * over `cond->if`, an arrow clause rewrites to
 * `((lambda (*cond-test*) (if *cond-test* (recipient *cond-test*) rest))
 * test)`, so the test evaluates exactly once and its value feeds the
 * recipient -- the naive `(if test (recipient test) rest)` would evaluate
 * the test twice. Plain clauses chain as before.
 */
public class WithArrow(
    global: Env,
) : Evaluator(global) {
    override fun condToIf(cond: CondE): Expr = arrowChain(cond.clauses)
}

/** The book's `cond->if` with arrow clauses folded into the chain. */
public fun arrowChain(clauses: PersistentList<CondClause>): Expr =
    if (clauses.isEmpty()) {
        VarE("false") // no else clause
    } else {
        val first = clauses.first()
        val rest = arrowChain(clauses.removingAt(0))
        when (first) {
            is CondClause.Else -> bodySequence(first.body)
            is CondClause.Clause -> arrowClause(first, rest)
        }
    }

/** One clause: an arrow binds the test value once and applies the
 * recipient to it; a plain clause tests as usual. */
private fun arrowClause(
    clause: CondClause.Clause,
    alternative: Expr,
): Expr =
    if (clause.body.size == 2 && clause.body[0] == VarE("=>")) {
        val recipient = clause.body[1]
        AppE(
            LambdaE(
                persistentListOf("*cond-test*"),
                null,
                persistentListOf(
                    IfE(
                        VarE("*cond-test*"),
                        AppE(recipient, persistentListOf(VarE("*cond-test*"))),
                        alternative,
                    ),
                ),
            ),
            persistentListOf(clause.test),
        )
    } else {
        IfE(clause.test, bodySequence(clause.body), alternative)
    }

/** The book's `sequence->exp`: one expression, `begin` when necessary. */
private fun bodySequence(body: PersistentList<Expr>): Expr = if (body.size == 1) body.first() else BeginE(body)
