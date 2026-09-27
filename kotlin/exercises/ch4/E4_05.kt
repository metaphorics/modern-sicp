// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.5

package sicp.ch4.exercises

import kotlinx.collections.immutable.PersistentList
import sicp.ch4.Evaluator
import sicp.runtime.CondClause
import sicp.runtime.CondE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.PendingSolution

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
    override fun condToIf(cond: CondE): Expr = throw PendingSolution()
}

/** The book's `cond->if` with arrow clauses folded into the chain. */
public fun arrowChain(clauses: PersistentList<CondClause>): Expr = throw PendingSolution()
