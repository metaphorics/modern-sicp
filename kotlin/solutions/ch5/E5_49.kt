// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.49: a persistent read-compile-execute-print
// session. Each turn submits new declarations to the compiler,
// executes its new entry on the same assembled machine, and captures
// only that turn's output. The second turn calls a function defined
// during the first, proving the earlier environment survives.

package sicp.ch5.solutions

import sicp.ch5.Compiler

/** Two turns with a shared machine and persistent compiled bindings. */
public fun readCompileExecutePrintRuns(): List<String> {
    val session = Compiler.session()
    val first =
        session
            .appendAndRun(
                """
                fun square(x: Long): Long {
                    return x * x
                }

                fun firstTurn(): Unit {
                    println(square(12L))
                }
                """.trimIndent(),
                "firstTurn",
            ).fold({ failure -> error("the first compiled turn failed: $failure") }, { it })
    val firstMachine = session.machine ?: error("the first turn has no machine")
    val second =
        session
            .appendAndRun(
                """
                fun twice(x: Long): Long {
                    return x * 2L
                }

                fun secondTurn(): Unit {
                    println(twice(square(21L)))
                }
                """.trimIndent(),
                "secondTurn",
            ).fold({ failure -> error("the second compiled turn failed: $failure") }, { it })
    return listOf(
        "first turn: ${first.output.trimEnd()}",
        "second turn: ${second.output.trimEnd()}",
        "the two turns share one machine: ${firstMachine === session.machine}",
    )
}
