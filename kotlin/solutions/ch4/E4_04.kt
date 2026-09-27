// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.4

package sicp.ch4.solutions

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.SchemeError
import sicp.runtime.VBool
import sicp.runtime.Value
import sicp.runtime.VarE
import sicp.runtime.isTrue

/**
 * Exercise 4.4: `and` and `or` as special forms, evaluated directly so the
 * operands re-enter the full evaluator at every depth. Short circuit is the
 * specification: the first false operand stops `and` at #f, the first true
 * value is `or`'s answer, and a later operand that would raise never
 * evaluates. `(and)` is #t, `(or)` is #f, and the last operand is the tail
 * answer, so `(and 1 2 3)` is 3.
 */
public class WithAndOr(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep {
        if (expr is AppE) {
            when ((expr.operator as? VarE)?.name) {
                "and" -> return andStep(expr.operands, env)
                "or" -> return orStep(expr.operands, env)
            }
        }
        return super.step(expr, env)
    }

    /** `(and e ...)`: the first false operand answers #f, no operands
     * answer #t, the last operand is the tail answer. */
    context(r: Raise<SchemeError>)
    private fun andStep(
        operands: PersistentList<Expr>,
        env: Env,
    ): EvalStep {
        if (operands.isEmpty()) return EvalStep.Done(VBool(true))
        for (i in 0 until operands.size - 1) {
            if (!isTrue(eval(operands[i], env))) return EvalStep.Done(VBool(false))
        }
        return EvalStep.Continue(operands.last(), env)
    }

    /** `(or e ...)`: the first true operand answers itself, no operands
     * answer #f, the last operand is the tail answer. */
    context(r: Raise<SchemeError>)
    private fun orStep(
        operands: PersistentList<Expr>,
        env: Env,
    ): EvalStep {
        if (operands.isEmpty()) return EvalStep.Done(VBool(false))
        for (i in 0 until operands.size - 1) {
            val v = eval(operands[i], env)
            if (isTrue(v)) return EvalStep.Done(v)
        }
        return EvalStep.Continue(operands.last(), env)
    }
}
