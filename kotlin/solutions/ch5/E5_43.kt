// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.43: scan out internal definitions.

package sicp.ch5.solutions

import sicp.ch5.CompilerConfig

private const val INTERNAL_DEFINITIONS = "(define (f) (define a 1) (define b 2) (+ a b))"
private const val BODY_FORM = "(lambda () (define a 1) (define b 2) (+ a b))"

/** Compares the emitted body shapes and executes the scanned procedure. */
public fun scanOutShapes(): List<String> {
    val plain = compiledStatements(CompilerConfig(), BODY_FORM)
    val scanned = compiledStatements(CompilerConfig(scanOut = true), BODY_FORM)
    val answer = valuesOf(runCompiled(CompilerConfig(scanOut = true), INTERNAL_DEFINITIONS, "(f)"))
    return listOf(
        "plain: marker=${plain.any { it.contains("*unassigned*") }}, define=${plain.any { it.contains("define-variable!") }}",
        "scanned: marker=${scanned.any { it.contains("*unassigned*") }}, define=${scanned.any { it.contains("define-variable!") }}",
        "scanned run: ${answer.lastOrNull()}",
    )
}
