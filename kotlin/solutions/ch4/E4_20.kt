// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20

package sicp.ch4.solutions

import arrow.core.raise.Raise
import kotlinx.collections.immutable.toPersistentList
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.LetBinding
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.VNil
import sicp.runtime.Value
import sicp.runtime.VarE

// Exercise 4.20: `letrec` as a derived expression. The parser leaves every
// `letrec` an application of the head symbol, so `step` recognizes
// `(letrec ((name init) ...) body ...)` and lowers it exactly like 4.16's
// scan-out: one `let` reserving every name with `*unassigned*`, one `set!`
// per name in source order, then the body. The derivation answers part (b):
// a plain `let` evaluates its initializers before any frame holds the
// names, so a recursive initializer would read the outer binding or fail
// unbound, while `letrec` reserves the names first and only the genuinely
// premature reads fail the typed check.

/** The `letrec` form's bindings and body, destructured out of the
 * parser's application shapes. */
context(r: Raise<SchemeError>)
internal fun letrecParts(expr: AppE): Pair<List<LetBinding>, List<Expr>> {
    if (expr.operands.isEmpty()) r.raise(SchemeError.Parse("bad letrec form: $expr"))
    val body = expr.operands.drop(1)
    if (body.isEmpty()) r.raise(SchemeError.Parse("bad letrec form: $expr"))
    return letrecBindings(expr.operands.first()) to body
}

/** The binding pairs: `(name init)` applications; an empty binding list is
 * the empty-list literal. */
context(r: Raise<SchemeError>)
private fun letrecBindings(operand: Expr): List<LetBinding> =
    when (operand) {
        is LitE -> {
            if (operand.v is VNil) {
                emptyList()
            } else {
                r.raise(SchemeError.Parse("bad letrec bindings: $operand"))
            }
        }

        is AppE -> {
            listOf(bindingOf(operand.operator)) + operand.operands.map { bindingOf(it) }
        }

        else -> {
            r.raise(SchemeError.Parse("bad letrec bindings: $operand"))
        }
    }

/** One `(name init)` pair as the parser shaped it. */
context(r: Raise<SchemeError>)
private fun bindingOf(expr: Expr): LetBinding {
    val application = expr as? AppE ?: r.raise(SchemeError.Parse("bad letrec binding: $expr"))
    val name = application.operator as? VarE ?: r.raise(SchemeError.Parse("bad letrec binding: $expr"))
    if (application.operands.size != 1) r.raise(SchemeError.Parse("bad letrec binding: $expr"))
    return LetBinding(name.name, application.operands.first())
}

/** The text strategy: reservations, then one `set!` per name, then the body. */
context(r: Raise<SchemeError>)
internal fun letrecToLet(expr: AppE): Expr {
    val (bindings, body) = letrecParts(expr)
    return LetE(
        bindings.map { LetBinding(it.name, LitE(unassignedMarker)) }.toPersistentList(),
        (bindings.map { SetE(it.name, it.value) } + body).toPersistentList(),
    )
}

/** The 4.20 evaluator: `letrec` lowers to the reserved `let`, and reads of
 * still-reserved names fail the typed premature-read check. */
public class WithLetrec(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep {
        if (expr is AppE) {
            val operator = expr.operator
            if (operator is VarE && operator.name == "letrec") {
                return EvalStep.Continue(letrecToLet(expr), env)
            }
        }
        return super.step(expr, env)
    }

    context(r: Raise<SchemeError>)
    override fun lookupVariable(
        name: String,
        env: Env,
    ): Value = requireAssigned(name, env.lookup(name))
}

/** The statement's mutual recursion under `letrec`, applied to 10. => "#t\n" */
public fun letrecEvenOddTranscript(): String =
    transcriptOn(
        ::WithLetrec,
        """
        (define (f n)
          (letrec ((even? (lambda (k) (if (= k 0) true (odd? (- k 1)))))
                   (odd? (lambda (k) (if (= k 0) false (even? (- k 1))))))
            (even? n)))
        (f 10)
        """.trimIndent(),
    )

/** The factorial recursion bound by one `letrec`. => "3628800\n" */
public fun letrecFactTranscript(): String =
    transcriptOn(
        ::WithLetrec,
        """
        (letrec ((fact (lambda (n) (if (= n 1) 1 (* n (fact (- n 1)))))))
          (fact 10))
        """.trimIndent(),
    )

/** An initializer that reads a fellow binding whose `set!` runs later:
 * the reservation makes that a typed premature read.
 * => "Error: type mismatch: y is read before it is assigned\n" */
public fun letrecPrematureReadTranscript(): String =
    transcriptOn(
        ::WithLetrec,
        """
        (letrec ((x (+ y 1)) (y 2)) x)
        """.trimIndent(),
    )

/** Part (b): the same recursion under a plain `let` -- the lambda captured
 * before the frame existed, so the recursive call finds no `fact` anywhere.
 * => "Error: unbound variable: fact\n" */
public fun plainLetUnboundTranscript(): String =
    transcriptOn(
        ::Evaluator,
        """
        (let ((fact (lambda (n) (if (= n 1) 1 (* n (fact (- n 1)))))))
          (fact 10))
        """.trimIndent(),
    )
