// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26

package sicp.ch4.solutions

import arrow.core.raise.Raise
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.ch4.LazyEvaluator
import sicp.ch4.lazyTranscriptOn
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.IfE
import sicp.runtime.SchemeError
import sicp.runtime.VarE

// Exercise 4.26: two implementations of `unless`, and what each costs.
// Ben's side: `unless` as a derived expression -- [WithUnlessDerived]
// recognizes `(unless condition usual-value exceptional-value)` in `step`
// and lowers it to `(if condition exceptional-value usual-value)`, so the
// untouched arm never evaluates. But the derivation is syntax: nothing is
// bound, so using `unless` as a value -- `(apply unless ...)` -- fails
// unbound, and no higher-order procedure can take it. Alyssa's side: under
// the lazy evaluator `unless` stays an ordinary procedure whose arms are
// thunks, so it composes: `map` maps it, `apply` applies it, and the
// unchosen arm's thunk is simply never forced.

/** Ben's evaluator: `unless` as a derived expression on the strict base. */
public class WithUnlessDerived(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep {
        val lowered = unlessLowering(expr)
        return if (lowered == null) super.step(expr, env) else EvalStep.Continue(lowered, env)
    }
}

/** The derived-expression rewrite, or null when [expr] is not a three-arm
 * `unless` application. */
internal fun unlessLowering(expr: Expr): Expr? {
    val application = expr as? AppE ?: return null
    val operator = application.operator as? VarE ?: return null
    if (operator.name != "unless" || application.operands.size != 3) return null
    val condition = application.operands[0]
    val usual = application.operands[1]
    val exceptional = application.operands[2]
    return IfE(condition, exceptional, usual)
}

/** Ben's derivation on the armed call: the chosen arm answers, the other
 * never evaluates. => "42\n" */
public fun unlessDerivedTranscript(): String =
    transcriptOn(
        ::WithUnlessDerived,
        """
        (unless (= 1 1) (/ 1 0) 42)
        """.trimIndent(),
    )

/** Alyssa's objection, pinned: the derived `unless` is syntax, so its name
 * is not a value. => "Error: unbound variable: unless\n" */
public fun unlessDerivedValueUseTranscript(): String =
    transcriptOn(
        ::WithUnlessDerived,
        """
        (apply unless '(#f 1 2))
        """.trimIndent(),
    )

/** Alyssa's implementation: an ordinary procedure under delayed arguments.
 * The unchosen arm is a thunk that is never forced. => "42\n" */
public fun unlessLazyProcedureTranscript(): String =
    lazyTranscriptOn(
        ::LazyEvaluator,
        """
        ${UNLESS_DEFINITION}
        (unless #f 42 (/ 1 0))
        """.trimIndent(),
    )

/** The higher-order composition the procedure keeps: `map` over
 * `unless`-armed triples, the exceptional arm never forced.
 * => "(42 7)\n" */
public fun unlessLazyMappedTranscript(): String =
    lazyTranscriptOn(
        ::LazyEvaluator,
        """
        ${UNLESS_DEFINITION}
        (map (lambda (t) (unless (car t) (cadr t) (caddr t)))
             '((#f 42 (/ 1 0)) (#f 7 (/ 1 0))))
        """.trimIndent(),
    )

/** `apply` takes the procedure value the special form could never name;
 * the delayed argument list stays lazy past the call.
 * => "7\n" */
public fun unlessLazyApplyTranscript(): String =
    lazyTranscriptOn(
        ::LazyEvaluator,
        """
        ${UNLESS_DEFINITION}
        (apply unless '(#f 7 (/ 1 0)))
        """.trimIndent(),
    )

/** The statement's `unless`, as a lazy procedure. */
private const val UNLESS_DEFINITION: String =
    """(define (unless condition usual-value exceptional-value)
  (if condition exceptional-value usual-value))"""
