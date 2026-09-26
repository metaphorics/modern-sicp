// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.38: open-coded primitive calls and n-ary folds.

package sicp.ch5.solutions

import sicp.ch5.CompilerConfig

private val factorial38 = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))"

/** Measures the open-coded factorial and runs binary and n-ary arithmetic. */
public fun openCodedRuns(): List<String> {
    val plainCount = compileCounts(CompilerConfig(), factorial38).first
    val open = CompilerConfig(openCode = true)
    val openCount = compileCounts(open, factorial38).first
    val factorial = valuesOf(runCompiled(open, factorial38, "(factorial 5)"))
    val sum = valuesOf(runCompiled(open, "(+ 1 2 3 4)", ""))
    val less = valuesOf(runCompiled(open, "(< 1 2)", ""))
    return listOf(
        "plain compilation: $plainCount statements",
        "open-coded compilation: $openCount statements",
        "factorial 5: ${factorial.joinToString(" ")}",
        "(+ 1 2 3 4): ${sum.joinToString(" ")}",
        "(< 1 2): ${less.joinToString(" ")}",
    )
}
