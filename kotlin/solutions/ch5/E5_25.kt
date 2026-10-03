// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.25: modify the evaluator to use normal-order
// evaluation. In this edition normal order is the named Lazy experiment
// of contract section 4.1: the same session runs in mode Lazy, where
// compound arguments are delayed and forced on reference, and the
// forcing instrument counts force attempts and distinct computations.
// The exercise's observable is that an unused argument is never
// evaluated and a thunk computes once, beside the session's answers.

package sicp.ch5.solutions

import sicp.ch4.LazyModule

/** The normal-order session: `first` never touches its second argument,
 *  and one thunk is forced twice. */
private val normalOrderSession: String =
    """
    var forces: Long = 0L

    fun slow(value: Long): Long {
        forces = forces + 1L
        return value
    }

    fun first(a: Long, b: Long): Long {
        return a
    }

    fun main() {
        println(first(120L, slow(42L)))
        val delayed: Thunk<Long> = thunk { slow(42L) }
        println(force(delayed))
        println(force(delayed))
        println(forces)
    }
    """.trimIndent()

/** The session's answers beside the two normal-order verdicts: the
 *  unused argument never ran (two written calls, one computation) and
 *  the thunk computed exactly once across its two forces. */
public fun normalOrderRuns(): List<String> {
    val run =
        LazyModule.run(normalOrderSession).fold(
            { error -> error("the normal-order session failed: $error") },
            { it },
        )
    val outputs =
        run.result.output
            .split("\n")
            .filter { it.isNotEmpty() }
    val forces = outputs.lastOrNull { it.toLongOrNull() != null }
    return outputs +
        listOf(
            "unused argument not evaluated: ${forces == "1"}",
            "thunk computed once: ${run.forcing.computations == 1L}",
        )
}
