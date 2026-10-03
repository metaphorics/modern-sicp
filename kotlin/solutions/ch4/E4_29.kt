// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.29

package sicp.ch4.solutions

import sicp.ch4.LazyModule

// Exercise 4.29: what memoization buys. The counting session is the
// demonstration itself: `id` counts its computations, and the two probes
// differ only in how a demand reaches the computation. Memoized, the
// parameter is one transparent thunk per argument: `square` reads its
// parameter twice and the second read answers from the memo, so the
// session counts 1 then 2. Without the memo every demand re-runs the
// computation: the same two calls count 2 then 5 -- a program whose
// delayed arguments are used more often than they are forced pays
// multiplicatively. The cold probe expresses call-by-name honestly in
// guest code: the delayed computation is a function value re-invoked at
// every demand, so the memo cannot hide the recomputation.

/** The counting session under memoized delay: `square` then `cube` count
 * 1 and 2. => "100\n1\n1000\n2\n" */
internal val MEMOIZED_COUNTS_PROGRAM: String =
    """
var count: Long = 0L

fun id(x: Long): Long {
    count = count + 1L
    return x
}

fun square(x: Long): Long = x * x

fun cube(x: Long): Long = x * x * x

fun main() {
    println(square(id(10L)))
    println(count)
    println(cube(id(10L)))
    println(count)
}
    """.trimIndent()

/** The counting session with every demand re-invoking the computation:
 * `squareCold` then `cubeCold` count 2 and 5. => "100\n2\n1000\n5\n" */
internal val COLD_COUNTS_PROGRAM: String =
    """
var count: Long = 0L

fun id(x: Long): Long {
    count = count + 1L
    return x
}

fun squareCold(get: () -> Long): Long = get() * get()

fun cubeCold(get: () -> Long): Long = get() * get() * get()

fun main() {
    println(squareCold { id(10L) })
    println(count)
    println(cubeCold { id(10L) })
    println(count)
}
    """.trimIndent()

/** Memoized thunks: `(square (id 10))` then `(cube (id 10))` count 1 and
 * 2. => "100\n1\n1000\n2\n" */
public fun memoizedCountsTranscript(): String = outcomeText(LazyModule.run(MEMOIZED_COUNTS_PROGRAM).map { it.result })

/** The same session with a recomputation at every demand: the counts read
 * 2 and 5. => "100\n2\n1000\n5\n" */
public fun unmemoizedCountsTranscript(): String = outcomeText(LazyModule.run(COLD_COUNTS_PROGRAM).map { it.result })
