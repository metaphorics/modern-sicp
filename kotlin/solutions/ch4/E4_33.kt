// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.33

package sicp.ch4.solutions

import arrow.core.raise.Raise
import kotlinx.collections.immutable.persistentListOf
import sicp.ch4.EvalStep
import sicp.ch4.LazyEvaluator
import sicp.ch4.lazyTranscriptOn
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.LambdaE
import sicp.runtime.LitE
import sicp.runtime.QuoteE
import sicp.runtime.SchemeError
import sicp.runtime.VPair
import sicp.runtime.Value
import sicp.runtime.VarE

// Exercise 4.33: quotes under the lazy regime. With the procedural pairs
// installed, `(car '(a b c))` fails: the quote is an ordinary pair of
// data, and `car` applies its `z` to it -- data is not applicable. The
// fix lifts the quote: [WithLazyQuote] rewrites every quoted pair into the
// expression `(lambda (m) (m <car> <cdr>))`, the same shape the
// object-language `cons` builds, with nested pairs lifted recursively and
// atoms left as data. Quoted lists then ARE lazy lists, and the list
// operations of the section run on them.

/** The evaluator whose quotes build lazy pairs. */
public class WithLazyQuote(
    global: Env,
) : LazyEvaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep {
        val lifted = if (expr is QuoteE) liftQuoteExpr(expr.datum) else null
        return if (lifted == null) super.step(expr, env) else EvalStep.Continue(lifted, env)
    }
}

/** The procedural-pair expression for one quoted pair, or null for data
 * that stays ordinary. */
internal fun liftQuoteExpr(datum: Value): Expr? {
    if (datum !is VPair) return null
    val body =
        AppE(
            VarE("m"),
            persistentListOf(quotedPart(datum.car), quotedPart(datum.cdr)),
        )
    return LambdaE(persistentListOf("m"), null, persistentListOf(body))
}

/** A nested pair lifts recursively; an atom stays self-evaluating data. */
private fun quotedPart(v: Value): Expr = liftQuoteExpr(v) ?: LitE(v)

/** The statement's probe with data quotes: the procedural `car` receives
 * an ordinary pair. => "Error: not a procedure: (a b c)\n" */
public fun plainQuoteCarTranscript(): String =
    lazyTranscriptOn(
        ::LazyEvaluator,
        """
        ${PROCEDURAL_LISTS}
        (car '(a b c))
        """.trimIndent(),
    )

/** Lifted quotes: the quoted list is a lazy pair, `car` answers.
 * => "a\n" */
public fun lazyQuoteCarTranscript(): String =
    lazyTranscriptOn(
        ::WithLazyQuote,
        """
        ${PROCEDURAL_LISTS}
        (car '(a b c))
        """.trimIndent(),
    )

/** The section's list operations run on quoted lists. => "d\n" */
public fun lazyQuoteListRefTranscript(): String =
    lazyTranscriptOn(
        ::WithLazyQuote,
        """
        ${PROCEDURAL_LISTS}
        (define (list-ref items n)
          (if (= n 0)
              (car items)
              (list-ref (cdr items) (- n 1))))
        (list-ref '(a b c d) 3)
        """.trimIndent(),
    )

/** The section's procedural pairs, as object-language definitions. */
private const val PROCEDURAL_LISTS: String =
    """(define (cons x y) (lambda (m) (m x y)))
(define (car z) (z (lambda (p q) p)))
(define (cdr z) (z (lambda (p q) q)))"""
