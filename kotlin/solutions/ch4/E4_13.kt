// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.13

package sicp.ch4.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.ch4.OutputSink
import sicp.ch4.parseProgram
import sicp.ch4.printValue
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.AppE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.SchemeError
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.VarE

// Exercise 4.13: `make-unbound!` removes a binding from the environment.
// [WithUnbound] checks for the form in `step` (it arrives as an
// application whose operator names `make-unbound!`), takes the operand's
// name unevaluated -- the variable may be about to stop existing -- and
// drops the binding from the FIRST frame only. First-frame-only is the
// reading that keeps `make-unbound!` a frame operation, symmetric with
// `define`, which also touches just one frame: unbinding reaches as far
// as `define` binds. A name absent from the first frame is left alone and
// the form answers `ok`, so unbind is idempotent; the shadow pin shows the
// outer binding becoming visible again after the shadow is removed.

/** The evaluator with the `make-unbound!` special form. */
public class WithUnbound(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep {
        if (expr is AppE) {
            val head = expr.operator as? VarE
            if (head?.name == "make-unbound!") {
                val name =
                    (expr.operands.singleOrNull() as? VarE)?.name
                        ?: r.raise(SchemeError.Parse("bad make-unbound! form"))
                env.frame = env.frame.removing(name)
                return EvalStep.Done(VSym("ok"))
            }
        }
        return super.step(expr, env)
    }
}

/** Unbind the global `x`, then fail to find it. */
private val UNBOUND_PROGRAM: String =
    """
    (define x 3)
    x
    (make-unbound! x)
    x
    """.trimIndent()

/** Unbind a shadowing `x` inside a body: the outer `x` shows through
 * again, and the global binding was never touched. */
private val SHADOW_PROGRAM: String =
    """
    (define x 1)
    (define (shadow!) (define x 2) (make-unbound! x) x)
    (shadow!)
    x
    """.trimIndent()

/** An absent name unbinds to `ok`; the first frame is left unchanged. */
private const val ABSENT_PROGRAM: String = "(make-unbound! nowhere)"

/** Runs [text] on a [WithUnbound] evaluator under the printer contract. */
private fun runUnbound(text: String): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    val evaluator = WithUnbound(env)
    either {
        for (expr in parseProgram(readProgram(text))) {
            if (expr is DefineE) {
                evaluator.eval(expr, env) // a define prints nothing
                continue
            }
            sink.line(printValue(evaluator.eval(expr, env)))
        }
    }.fold(
        { e -> sink.line("Error: ${sicp.ch4.formatError(e)}") },
        { },
    )
    return sink.toString()
}

/** Unbinding removes the first frame's binding; the next lookup faults.
 * => "3\nok\nError: unbound variable: x\n" */
public fun unboundTranscript(): String = runUnbound(UNBOUND_PROGRAM)

/** After the shadow is unbound, the outer `x` is visible again.
 * => "1\n1\n" */
public fun unboundShadowTranscript(): String = runUnbound(SHADOW_PROGRAM)

/** Unbinding an absent name is a no-op answering `ok`. => "ok\n" */
public fun unboundAbsentTranscript(): String = runUnbound(ABSENT_PROGRAM)
