// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26a

package sicp.ch4.solutions

import arrow.core.raise.Raise
import kotlinx.collections.immutable.toPersistentList
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.ch4.LazyEvaluator
import sicp.ch4.lazyTranscriptOn
import sicp.runtime.AppE
import sicp.runtime.BeginE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.IfE
import sicp.runtime.SchemeError
import sicp.runtime.VarE

// Exercise 4.26a (added by this edition): `when` as a derived expression,
// the mirror of 4.26's `unless`. The derivation is macro-style: one rewrite
// to forms the evaluator already has -- `(when condition body ...)`
// lowers to `(if condition (begin body ...) false)`, the `false` variable
// standing where the parser's missing alternative stands. [WithWhen]
// checks for the application shape in `step` and falls back to the lazy
// chain for everything else, so the derivation composes with the section's
// delay rules; a before/after trace pins that the name is unbound before
// the derivation and answers after it.

/** The `when` evaluator: the derived form over the lazy base. */
public class WithWhen(
    global: Env,
) : LazyEvaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep {
        val lowered = whenLowering(expr)
        return if (lowered == null) super.step(expr, env) else EvalStep.Continue(lowered, env)
    }
}

/** The derived-expression rewrite `(when c body ...)` ->
 * `(if c (begin body ...) false)`, or null when [expr] is not a `when`
 * application. */
internal fun whenLowering(expr: Expr): Expr? {
    val application = expr as? AppE ?: return null
    val operator = application.operator as? VarE ?: return null
    if (operator.name != "when" || application.operands.isEmpty()) return null
    val condition = application.operands.first()
    val body = application.operands.drop(1)
    val bodyExpr = if (body.size == 1) body.first() else BeginE(body.toPersistentList())
    return IfE(condition, bodyExpr, VarE("false"))
}

/** The before trace: without the derivation, `when` is an application of
 * an unbound name. => "Error: unbound variable: when\n" */
public fun whenBeforeTranscript(): String =
    transcriptOn(
        ::Evaluator,
        """
        (when (> 3 2) 'yes)
        """.trimIndent(),
    )

/** The after trace: the derived `when` evaluates. => "yes\n" */
public fun whenAfterTranscript(): String =
    lazyTranscriptOn(
        ::WithWhen,
        """
        (when (> 3 2) 'yes)
        """.trimIndent(),
    )

/** A false condition with no else arm answers the parser's missing
 * alternative. => "#f\n" */
public fun whenNoElseTranscript(): String =
    lazyTranscriptOn(
        ::WithWhen,
        """
        (when (= 1 2) 'yes 'no)
        """.trimIndent(),
    )

/** A multi-expression body runs as one sequence, answering the last.
 * => "3\n" */
public fun whenBodySequenceTranscript(): String =
    lazyTranscriptOn(
        ::WithWhen,
        """
        (when #t 1 2 3)
        """.trimIndent(),
    )
