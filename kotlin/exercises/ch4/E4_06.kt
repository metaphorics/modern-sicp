// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.6

package sicp.ch4.exercises

import sicp.ch4.Evaluator
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.LetE
import sicp.runtime.PendingSolution

/**
 * Exercise 4.6: `let` as a derived expression. [letRewrite] rewrites
 * `(let ((x 3) (y 4)) body)` to `((lambda (x y) body) 3 4)` and the
 * evaluator derives every `let` through it. The rewrite fixes the scoping
 * rule: the application evaluates the inits as operands, in the outer
 * environment -- with x bound to 5 outside, `(let ((x 3) (y x)) y)` is 5,
 * not 3.
 */
public class WithLetDerived(
    global: Env,
) : Evaluator(global) {
    override fun letToCombination(expr: LetE): Expr = throw PendingSolution()
}

/** The book's `let->combination`: parameters, body, then the inits as the
 * operands of the application. */
public fun letRewrite(expr: LetE): Expr = throw PendingSolution()
