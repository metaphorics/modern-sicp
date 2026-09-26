// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.44: open coding and shadowing.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.CompilerConfig
import sicp.ch5.CompilerState
import sicp.ch5.compileProgram
import sicp.ch5.openCodedPrimitives
import sicp.ch5.renderStmt

private fun openCodedCount(source: String): Int {
    val cfg = CompilerConfig(openCode = true)
    val statements =
        either { compileProgram(cfg, CompilerState(), readForms(source)).stmts.map(::renderStmt) }
            .fold({ error("compile failed: $it") }, { it })
    return statements.count { it.contains("(op +) (reg arg1)") || it.contains("(op *) (reg arg1)") }
}

/** Counts shadowed and free open-coded operations, then records a rebind warning. */
public fun openCodeShadowingCounts(): List<String> {
    val shadowed = "(lambda (+ * a b x y) (+ (* a x) (* b y)))"
    val free = "(lambda (a b x y) (+ (* a x) (* b y)))"
    val state = CompilerState()
    either { compileProgram(CompilerConfig(openCode = true), state, readForms("(define (+ a b) a)")) }
        .fold({ error("compile failed: $it") }, { it })
    return listOf(
        "shadowed parameters: ${openCodedCount(shadowed)} open-coded operations",
        "free names: ${openCodedCount(free)} open-coded operations",
        "warnings: ${state.warnings.joinToString()}",
        "open-coded names: ${openCodedPrimitives.joinToString(" ")}",
    )
}
