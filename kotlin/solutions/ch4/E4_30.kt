// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.30

package sicp.ch4.solutions

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import sicp.ch4.EvalStep
import sicp.ch4.LazyEvaluator
import sicp.ch4.lazyTranscriptOn
import sicp.runtime.BeginE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.SchemeError

// Exercise 4.30: does `eval-sequence` force? The text's rule -- inherited
// here from the 4.1 evaluator, which [LazyEvaluator] does not touch --
// evaluates every non-final expression with `eval` and forces only what a
// demand site forces. Cy's rule forces every non-final expression with
// `actual-value`; [CySequenceLazy] implements it at both sequence sites:
// explicit `begin` forms and the body prefix of an applied compound
// procedure, which is the same `eval-sequence` in this substrate.

/** Cy's evaluator: every non-final sequence expression is forced. */
public class CySequenceLazy(
    global: Env,
) : LazyEvaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep =
        when (expr) {
            is BeginE -> {
                for (i in 0 until expr.actions.size - 1) actualValue(expr.actions[i], env)
                EvalStep.Continue(expr.actions.last(), env)
            }

            else -> {
                super.step(expr, env)
            }
        }

    context(r: Raise<SchemeError>)
    override fun evalBodyPrefix(
        body: PersistentList<Expr>,
        env: Env,
    ) {
        for (i in 0 until body.size - 1) actualValue(body[i], env)
    }
}

/** Ben's example under the text's rule: `display` and `newline` are strict
 * primitives, so their operands force at application and the three
 * elements print, `done` last. => "\n57\n321\n88done\n" */
public fun forEachTextRuleTranscript(): String =
    lazyTranscriptOn(
        ::LazyEvaluator,
        FOR_EACH_PROGRAM,
    )

/** The same session under Cy's rule: forcing the non-final expressions
 * changes nothing, because each one is an application whose own demand
 * sites force. => "\n57\n321\n88done\n" */
public fun forEachCyRuleTranscript(): String =
    lazyTranscriptOn(
        ::CySequenceLazy,
        FOR_EACH_PROGRAM,
    )

/** Cy's pair under the text's rule: `p1`'s `set!` runs (its own evaluation
 * forces the `cons` arguments), but `p2`'s non-final body expression `e`
 * evaluates to the delayed argument without forcing it, so the `set!`
 * inside never runs and `x` stays 1. => "(1 2)\n1\n" */
public fun p1P2TextRuleTranscript(): String =
    lazyTranscriptOn(
        ::LazyEvaluator,
        CY_PROGRAM,
    )

/** The same pair under Cy's rule: forcing `e` runs the delayed `set!`, so
 * `x` is the mutated pair by the time the final expression reads it.
 * => "(1 2)\n(1 2)\n" */
public fun p1P2CyRuleTranscript(): String =
    lazyTranscriptOn(
        ::CySequenceLazy,
        CY_PROGRAM,
    )

/** Ben's `for-each` session. */
private const val FOR_EACH_PROGRAM: String =
    """(define (for-each proc items)
  (if (null? items)
      'done
      (begin (proc (car items))
             (for-each proc (cdr items)))))
(for-each (lambda (x) (newline) (display x))
          (list 57 321 88))"""

/** Cy's `p1` and `p2`. */
private const val CY_PROGRAM: String =
    """(define (p1 x)
  (set! x (cons x '(2)))
  x)
(define (p2 x)
  (define (p e) e x)
  (p (set! x (cons x '(2)))))
(p1 1)
(p2 1)"""
