// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.15

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
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.SchemeError
import sicp.runtime.VBool
import sicp.runtime.VPrimitive
import sicp.runtime.Value

// Exercise 4.15: can `halts?` be written in the evaluator? [Bounded]
// answers honestly what a real `halts?` cannot: it carries a step budget
// in `step`, and a run that outlives its budget dies with the typed
// `MachineFault` instead of hanging. The book's dialogue runs with
// `halts?` installed as a primitive the host controls -- a stub whose
// canned answer the test flips. With the stub answering `#f`, `(try try)`
// takes the `'halts` branch and the run terminates: the program halted
// though `halts?` denied it. With the stub answering `#t`, the
// `'run-forever` branch spins until the budget fires: the program ran on
// though `halts?` affirmed it. Either canned answer is wrong about this
// input, which is the diagonal contradiction, stated with the budget as
// the witness instead of an infinite loop.

/** The evaluator that refuses to run forever: every `step` round costs
 * one of [budget] steps, and running out is a typed machine fault. */
public class Bounded(
    global: Env,
    /** The step budget of one run; generous for honest programs. */
    public val budget: Int = 1000,
) : Evaluator(global) {
    /** The steps taken so far. */
    public var steps: Int = 0
        private set

    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep {
        steps++
        if (steps > budget) {
            r.raise(SchemeError.MachineFault("step budget exhausted after $budget steps"))
        }
        return super.step(expr, env)
    }
}

/** The book's dialogue: `try` asks `halts?` about itself and, on yes,
 * runs forever. */
private val HALTS_PROGRAM: String =
    """
    (define (run-forever) (run-forever))
    (define (try p) (if (halts? p p) (run-forever) 'halts))
    (try try)
    """.trimIndent()

/** Runs the dialogue with `halts?` canned to [haltsAnswer], on a
 * [Bounded] evaluator under the printer contract. */
private fun boundedRun(haltsAnswer: Boolean): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    env.define("halts?", VPrimitive("halts?") { _ -> VBool(haltsAnswer) })
    val evaluator = Bounded(env)
    either {
        for (expr in parseProgram(readProgram(HALTS_PROGRAM))) {
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

/** The book's halts? dialogue on the bounded evaluator: `#f` halts,
 * `#t` exhausts the budget. */
public fun boundedTranscript(haltsAnswer: Boolean): String = boundedRun(haltsAnswer)
