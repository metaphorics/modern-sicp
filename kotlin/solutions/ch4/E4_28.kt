// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.28

package sicp.ch4.solutions

import arrow.core.raise.Raise
import sicp.ch4.EvalStep
import sicp.ch4.LazyEvaluator
import sicp.ch4.lazyTranscriptOn
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.SchemeError

// Exercise 4.28: why the operator forces. `apply` dispatches on the
// procedure VALUE in the operator position -- primitive versus compound --
// so what sits there must be an actual procedure, not the thunk an
// argument would be. In `((id +) 2 3)` the operator expression `(id +)`
// evaluates to `id`'s parameter, which the application delayed: forcing it
// yields the `+` primitive and the call answers 5. [UnforcedOperator] is
// the counterfactual the exercise asks for: it applies whatever `eval`
// returned without forcing, so the call dispatches on a thunk and fails
// the typed not-applicable fault, naming the thunk.

/** The counterfactual evaluator: no forcing in the operator position. */
public class UnforcedOperator(
    global: Env,
) : LazyEvaluator(global) {
    context(r: Raise<SchemeError>)
    override fun evalApplication(
        expr: AppE,
        env: Env,
    ): EvalStep {
        val procedure = eval(expr.operator, env) // the book's `eval`, not `actual-value`
        return applyDelaying(procedure, expr.operands, env)
    }
}

/** The operator forced: `((id +) 2 3)` dispatches on the `+` primitive.
 * => "5\n" */
public fun forcedOperatorTranscript(): String =
    lazyTranscriptOn(
        ::LazyEvaluator,
        """
        (define (id x) x)
        ((id +) 2 3)
        """.trimIndent(),
    )

/** The operator unforced: the call dispatches on the delayed operand.
 * => "Error: not a procedure: #[thunk]\n" */
public fun unforcedOperatorTranscript(): String =
    lazyTranscriptOn(
        ::UnforcedOperator,
        """
        (define (id x) x)
        ((id +) 2 3)
        """.trimIndent(),
    )
