// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.15

package sicp.ch4.solutions

import sicp.ch4.Direct

// Exercise 4.15: the halting theorem. No total decider exists -- the
// proof is prose and lives in the rationale. The probe shows what a
// step budget can and cannot do: a budgeted simulation answers `halts`
// for a program that finishes inside the budget, and abstains with
// `unknown` past it, for the diagonal at every budget. The budget
// decides nothing; it only cuts the simulation off.

// Exercise 4.15: a step budget abstains past its limit, decides nothing.

/** Budgeted simulation: `halts` inside the budget, `unknown` past it. */
internal val HALTING_SOURCE: String =
    """
fun quick(limit: Long): String {
    var depth = 0L
    while (depth <= limit) {
        if (depth >= 3L) {
            return "halts"
        }
        depth = depth + 1L
    }
    return "unknown after " + showLong(limit) + " steps"
}

fun diagonal(limit: Long): String {
    var depth = 0L
    while (depth <= limit) {
        depth = depth + 1L
    }
    return "unknown after " + showLong(limit) + " steps"
}

fun decide(program: Long, limit: Long): String {
    if (program == 0L) {
        return quick(limit)
    }
    return diagonal(limit)
}
    """.trimIndent()

/** The quick program halts; the diagonal abstains at every budget.
 * => "halts\nunknown after 200 steps\nunknown after 300 steps\n" */
public fun haltingProbeTranscript(): String =
    outcomeText(
        Direct.run(
            KERNEL_SOURCE + "\n" + HALTING_SOURCE + "\n" +
                """
fun main() {
    println(decide(0L, 200L))
    println(decide(1L, 200L))
    println(decide(1L, 300L))
}
                """.trimIndent(),
        ),
    )
