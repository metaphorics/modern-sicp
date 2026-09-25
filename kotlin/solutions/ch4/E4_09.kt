// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.9

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
import sicp.runtime.IfE
import sicp.runtime.LambdaE
import sicp.runtime.LitE
import sicp.runtime.QuoteE
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.VNil
import sicp.runtime.VSym
import sicp.runtime.VarE

/**
 * Exercise 4.9: iteration constructs designed as derived expressions.
 * `(while test body...)` and `(until test body...)` each rewrite to a
 * set!-installed zero-argument loop procedure,
 * `((lambda (tag) (set! tag (lambda () round)) (tag)) 'tag)` with the round
 * being `(if test (begin body... (tag)) ())` for `while` and the mirrored
 * `(if test () (begin body... (tag)))` for `until`. The rewrite fixes both
 * properties that matter: the body re-enters the full evaluator at every
 * iteration, and the self-call sits in tail position, so the loop runs an
 * iterative process in constant host stack.
 */
public class WithLoops(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep {
        if (expr is AppE && expr.operator == VarE("while")) {
            return EvalStep.Continue(whileToCombination(expr), env)
        }
        if (expr is AppE && expr.operator == VarE("until")) {
            return EvalStep.Continue(untilToCombination(expr), env)
        }
        return super.step(expr, env)
    }
}

/** `(while test body...)`: repeat the body while the test holds; the
 * answer is the unspecified value. */
context(r: Raise<SchemeError>)
public fun whileToCombination(expr: AppE): Expr {
    if (expr.operands.isEmpty()) r.raise(SchemeError.Parse("bad while form: no test"))
    val test = expr.operands.first()
    val body = expr.operands.drop(1)
    val round =
        IfE(
            test,
            roundBody(body, "*while*"),
            LitE(VNil),
        )
    return loopCombination("*while*", round)
}

/** `(until test body...)`: repeat the body until the test holds; the
 * answer is the unspecified value. */
context(r: Raise<SchemeError>)
public fun untilToCombination(expr: AppE): Expr {
    if (expr.operands.isEmpty()) r.raise(SchemeError.Parse("bad until form: no test"))
    val test = expr.operands.first()
    val body = expr.operands.drop(1)
    val round =
        IfE(
            test,
            LitE(VNil),
            roundBody(body, "*until*"),
        )
    return loopCombination("*until*", round)
}

/** The next round: the body followed by the self-call, in tail position. */
private fun roundBody(
    body: List<Expr>,
    tag: String,
): Expr =
    if (body.isEmpty()) {
        AppE(VarE(tag), persistentListOf())
    } else {
        BeginE((body + AppE(VarE(tag), persistentListOf())).toPersistentList())
    }

/** `((lambda (tag) (set! tag (lambda () round)) (tag)) 'tag)`: the frame
 * the wrapper binds `tag` in is the one the body's self-calls resolve
 * through, which is why the set! shape is required. */
private fun loopCombination(
    tag: String,
    round: Expr,
): Expr =
    AppE(
        LambdaE(
            persistentListOf(tag),
            null,
            persistentListOf(
                SetE(tag, LambdaE(persistentListOf(), null, persistentListOf(round))),
                AppE(VarE(tag), persistentListOf()),
            ),
        ),
        persistentListOf(QuoteE(VSym(tag))),
    )
