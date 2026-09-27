// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.40: compile-time environment threading.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.CompilerConfig
import sicp.ch5.CompilerState
import sicp.ch5.compileProgram
import sicp.ch5.renderStmt

private const val NESTED_LAMBDA = "(define (f x y) (lambda (a b c d e) (lambda (y z) (+ x y z))))"

/** Reports each variable and the lexical frames visible at its reference. */
public fun compileTimeEnvDump(): List<String> {
    val rows = mutableListOf<String>()
    val cfg =
        CompilerConfig(trace = { frames, name ->
            rows.add("$name in ${frames.joinToString(" ") { frame -> frame.joinToString(" ", "(", ")") }}")
        })
    either {
        compileProgram(cfg, CompilerState(), readForms(NESTED_LAMBDA)).stmts.map(::renderStmt)
    }.fold({ error("compile failed: $it") }, { it })
    return rows
}
