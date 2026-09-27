// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.8

package sicp.ch4.solutions

import arrow.core.raise.Raise
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.LambdaE
import sicp.runtime.LetBinding
import sicp.runtime.LitE
import sicp.runtime.QuoteE
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.VarE
import sicp.runtime.cons

/**
 * Exercise 4.8: named `let`. The grammar's parser rejects the named shape
 * `(let name bindings body)` -- a symbol in the bindings slot -- so
 * [namedLetExpansion] renames it to `named-let` at the datum level, and the
 * parser delivers that as the application the step seam expects.
 * [namedLetToCombination] derives the form to the set!-based wrapper
 * `((lambda (name) (set! name (lambda bindings body)) (name inits))
 * '*named-let*)`: the wrapper's frame binds `name`, the `set!` installs the
 * loop procedure into that binding, and the body's recursive self-calls
 * resolve it there. A direct `(lambda (name) ...)` nesting would instead
 * shadow `name` in the application frame and leave the self-calls unbound.
 */
public class WithNamedLet(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep =
        if (expr is AppE && expr.operator == VarE("named-let")) {
            EvalStep.Continue(namedLetToCombination(expr), env)
        } else {
            super.step(expr, env)
        }
}

/** The datum-level parse seam: `(let name bindings body...)` becomes
 * `(named-let name bindings body...)`, recursively; quoted data passes
 * through untouched. */
public fun namedLetExpansion(datum: Value): Value =
    when {
        datum !is VPair -> {
            datum
        }

        datum.car == VSym("quote") -> {
            datum
        }

        datum.car == VSym("let") && datum.cdr is VPair && (datum.cdr as VPair).car is VSym -> {
            cons(VSym("named-let"), namedLetExpansion(datum.cdr))
        }

        else -> {
            cons(namedLetExpansion(datum.car), namedLetExpansion(datum.cdr))
        }
    }

/** The book's named-let derivation: the set!-based wrapper application. */
context(r: Raise<SchemeError>)
public fun namedLetToCombination(expr: AppE): Expr {
    if (expr.operands.size < 3) r.raise(SchemeError.Parse("bad named let form: too few parts"))
    val name = expr.operands[0] as? VarE ?: r.raise(SchemeError.Parse("bad named let name"))
    val bindings = bindingsOf(expr.operands[1], "named let")
    val loop =
        LambdaE(
            bindings.map { it.name }.toPersistentList(),
            null,
            expr.operands.drop(2).toPersistentList(),
        )
    val wrapper =
        LambdaE(
            persistentListOf(name.name),
            null,
            persistentListOf(
                SetE(name.name, loop),
                AppE(VarE(name.name), bindings.map { it.value }.toPersistentList()),
            ),
        )
    return AppE(wrapper, persistentListOf(QuoteE(VSym("*named-let*"))))
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
