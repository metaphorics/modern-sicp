// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.1

package sicp.ch4.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import kotlinx.collections.immutable.PersistentList
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
import sicp.runtime.Value

// Exercise 4.1: operand evaluation order. Kotlin already fixes the host
// order -- operands evaluate left to right -- so the base evaluator's
// `listOfValues` maps the operands in order and the question the Scheme
// text leaves open has a definite answer here. The exercise still
// matters: writing the operand walk ourselves makes the order a property
// of the evaluator's code, and the right-to-left variant shows exactly
// what the host had been deciding. The probe is the book's device:
// `(cons (note 1) (note 2))` with `note` pushing onto a recorded list.

/** The probe program: the cons prints `(1 . 2)`; `order` records the run. */
private val PROBE: String =
    """
    (define order '())
    (define (note x) (set! order (cons x order)) x)
    (cons (note 1) (note 2))
    order
    """.trimIndent()

/** Runs [text] on [evaluatorFactory]'s evaluator and returns the printer-
 * contract transcript: defines print nothing, values print one line each. */
private fun runOn(
    evaluatorFactory: (Env) -> Evaluator,
    text: String,
): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    val evaluator = evaluatorFactory(env)
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

/** The base evaluator's operand walk: the host's fixed left-to-right order.
 * => (1 . 2) then (2 1) */
public fun leftToRightTranscript(): String = runOn(::Evaluator, PROBE)

/**
 * The exercise's right-to-left `list-of-values`: [kotlin.collections.foldRight]
 * runs the operands last-first, consing each value onto the front of the
 * already-evaluated rest, so the operator's argument list ends up identical
 * while the evaluation order inverts.
 */
public class RightToLeft(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun listOfValues(
        operands: PersistentList<Expr>,
        env: Env,
    ): List<Value> = operands.foldRight(emptyList()) { operand, acc -> listOf(eval(operand, env)) + acc }
}

/** The right-to-left evaluator on the probe. => (1 . 2) then (1 2) */
public fun rightToLeftTranscript(): String = runOn(::RightToLeft, PROBE)
