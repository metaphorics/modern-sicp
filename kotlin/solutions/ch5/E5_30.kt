// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.30: error signaling inside the evaluator. In
// this edition the premise splits at the admission boundary: unbound
// names, arity mismatches and non-Boolean conditionals are rejected at
// admission, before any guest effect (contract sections 1 and 3.8), and
// the runtime traps ride `RunResult.error` with their contract
// categories. The lesson the exercise teaches holds throughout: a
// failure never kills the session, it is reported and the loop
// continues to the next program.

package sicp.ch5.solutions

import sicp.ch5.ExplicitControl
import sicp.guest.Admission
import sicp.guest.Mode

/** The three programs the old evaluator trapped at run time and the
 *  typed rules now refuse before execution. */
private val rejected: List<Pair<String, String>> =
    listOf(
        "unbound variable" to
            """
            fun main() {
                println(noSuchVariable)
            }
            """.trimIndent(),
        "arity mismatch" to
            """
            fun double(value: Long): Long {
                return value * 2L
            }

            fun main() {
                println(double(1L, 2L))
            }
            """.trimIndent(),
        "non-Boolean condition" to
            """
            fun main() {
                if (1L) {
                    println(1L)
                }
            }
            """.trimIndent(),
    )

/** The run-time trap the categories keep: integer division by zero. */
private val dividedSource: String =
    """
    fun divide(a: Long, b: Long): Long {
        return a / b
    }

    fun main() {
        println(divide(1L, 0L))
    }
    """.trimIndent()

/** The clean session that answers after every failure. */
private val cleanFactorialSource: String =
    """
    fun factorial(n: Long): Long {
        if (n == 1L) {
            return 1L
        }
        return n * factorial(n - 1L)
    }

    fun main() {
        println(factorial(5L))
    }
    """.trimIndent()

/** The session through its paces: three admission rejections with their
 *  verdicts, the run-time trap's category, and the clean factorial that
 *  still answers. */
public fun errorSignalingRuns(): List<String> {
    val lines = mutableListOf<String>()
    for ((name, source) in rejected) {
        val refused = Admission.admit(source, Mode.CORE).fold({ true }, { false })
        lines.add("$name rejected before effects: $refused")
    }
    val divided =
        ExplicitControl.run(dividedSource).fold(
            { error -> error("the dividing session did not admit: $error") },
            { it },
        )
    lines.add("operation failed: ${divided.error?.category ?: "no error"}")
    val clean =
        ExplicitControl.run(cleanFactorialSource).fold(
            { error -> error("the clean session did not admit: $error") },
            { it },
        )
    lines.add("clean factorial: ${clean.output.trim()}")
    return lines
}
