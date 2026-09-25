// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.6

package sicp.ch4.solutions

import kotlinx.collections.immutable.toPersistentList
import sicp.ch4.Evaluator
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.LambdaE
import sicp.runtime.LetE

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
    override fun letToCombination(expr: LetE): Expr = letRewrite(expr)
}

/** The book's `let->combination`: parameters, body, then the inits as the
 * operands of the application. */
public fun letRewrite(expr: LetE): Expr =
    AppE(
        LambdaE(expr.bindings.map { it.name }.toPersistentList(), null, expr.body),
        expr.bindings.map { it.value }.toPersistentList(),
    )
