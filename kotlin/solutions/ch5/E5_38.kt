// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.38: open-coded primitive calls and n-ary folds.

package sicp.ch5.solutions

import sicp.ch5.CompilerConfig

private val factorial38 = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))"

/** The compound-call operand shapes: an operand that calls a compiled
 *  procedure trashes the caller's arg1 and arg2 through the callee's
 *  own open-coded body, so a call sequence must claim both registers
 *  or the operand shields stay silent and the fold reads trash. */
private val shieldSources =
    listOf(
        "(define (f y) (* y 10)) (define x 4) (+ x (f 1))",
        "(define (f y) (* y 10)) (define x 4) (+ x (f 1) 3)",
        "(define (f y) (* y 10)) (define (g y) (+ y 100)) (+ (+ (f 1) (+ 2 3)) (+ (* 2 2) (g 1)))",
    )

/** Measures the open-coded factorial and runs binary and n-ary arithmetic. */
public fun openCodedRuns(): List<String> {
    val plainCount = compileCounts(CompilerConfig(), factorial38).first
    val open = CompilerConfig(openCode = true)
    val openCount = compileCounts(open, factorial38).first
    val factorial = valuesOf(runCompiled(open, factorial38, "(factorial 5)"))
    val sum = valuesOf(runCompiled(open, "(+ 1 2 3 4)", ""))
    val less = valuesOf(runCompiled(open, "(< 1 2)", ""))
    val nested = valuesOf(runCompiled(open, "(+ (* 2 3) (+ 4 5))", ""))
    val call = valuesOf(runCompiled(open, "(define (f) 40) (+ 1 2 (f))", ""))
    val asOperand = valuesOf(runCompiled(open, "(+ (+ 1 2 3) 4)", ""))
    val shielded = shieldSources.map { source -> "$source: ${valuesOf(runCompiled(open, source, "")).joinToString(" ")}" }
    return listOf(
        "plain compilation: $plainCount statements",
        "open-coded compilation: $openCount statements",
        "factorial 5: ${factorial.joinToString(" ")}",
        "(+ 1 2 3 4): ${sum.joinToString(" ")}",
        "(< 1 2): ${less.joinToString(" ")}",
        "(+ (* 2 3) (+ 4 5)): ${nested.joinToString(" ")}",
        "(define (f) 40) (+ 1 2 (f)): ${call.joinToString(" ")}",
        "(+ (+ 1 2 3) 4): ${asOperand.joinToString(" ")}",
    ) + shielded
}
