// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.7

package sicp.ch4.solutions

import arrow.core.raise.Raise
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.runtime.AppE
import sicp.runtime.BeginE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.LetBinding
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.SchemeError
import sicp.runtime.VNil
import sicp.runtime.VarE

/**
 * Exercise 4.7: `let*` as nested `let`s. [letStarToNestedLets] folds the
 * bindings right to left, `(let* ((x 3) (y (+ x 1))) body)` =>
 * `(let ((x 3)) (let ((y (+ x 1))) body))`, so each init sees the earlier
 * bindings of the same `let*`. Adding the one `step` clause is sufficient:
 * every derived `let` re-enters the evaluator at the next depth, which is
 * also what makes a `let*` nested inside another's body work.
 */
public class WithLetStar(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep =
        if (expr is AppE && expr.operator == VarE("let*")) {
            EvalStep.Continue(letStarToNestedLets(expr), env)
        } else {
            super.step(expr, env)
        }
}

/** The book's `let*->nested-lets`, folded right to left; zero bindings
 * leave the body as a sequence. The parsed `let*` arrives as the
 * application the parser leaves for it, bindings encoded as the application
 * chain `(name init) (name init)`. */
context(r: Raise<SchemeError>)
public fun letStarToNestedLets(expr: AppE): Expr {
    if (expr.operands.isEmpty()) r.raise(SchemeError.Parse("bad let* form: ${expr.operands}"))
    val body = expr.operands.drop(1)
    if (body.isEmpty()) r.raise(SchemeError.Parse("bad let* form: empty body"))
    var derived: Expr = bodyOf(body)
    for (binding in bindingsOf(expr.operands.first(), "let*").reversed()) {
        derived = LetE(persistentListOf(binding), persistentListOf(derived))
    }
    return derived
}

context(r: Raise<SchemeError>)
private fun bindingsOf(
    expr: Expr,
    form: String,
): List<LetBinding> =
    when (expr) {
        is LitE -> {
            if (expr.v is VNil) {
                emptyList()
            } else {
                r.raise(SchemeError.Parse("bad $form bindings: ${expr.v}"))
            }
        }

        is AppE -> {
            listOf(bindingOf(expr.operator, form)) + expr.operands.map { bindingOf(it, form) }
        }

        else -> {
            r.raise(SchemeError.Parse("bad $form bindings"))
        }
    }

context(r: Raise<SchemeError>)
private fun bindingOf(
    expr: Expr,
    form: String,
): LetBinding {
    val binding = expr as? AppE ?: r.raise(SchemeError.Parse("bad $form binding: $expr"))
    val name = binding.operator as? VarE ?: r.raise(SchemeError.Parse("bad $form binding name: $expr"))
    if (binding.operands.size != 1) r.raise(SchemeError.Parse("bad $form binding: $expr"))
    return LetBinding(name.name, binding.operands.first())
}

private fun bodyOf(body: List<Expr>): Expr = if (body.size == 1) body[0] else BeginE(body.toPersistentList())
