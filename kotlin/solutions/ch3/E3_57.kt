// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.57

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.streamTail

/**
 * Exercise 3.57: how many additions are performed when we compute the
 * n-th Fibonacci number using the definition of fibs based on
 * add-streams? With the memoized tail each element past the first two
 * costs exactly one addition, never repeated, so the count at index n
 * is n - 1 (0 for n <= 1). With a plain lambda delay every tail force
 * re-runs its zip, whose two argument streams re-derive the whole tree
 * below them before their heads are added; the count then grows
 * exponentially, like the tree-recursive Fibonacci of section 1.2.2.
 * The memoized side counts through the pair stream the local builder
 * ties together, each element carrying the Fibonacci value and the
 * additions performed so far; the unmemoized side walks the same
 * construction cold, paying the fresh derivation of every element.
 */
public fun fibAdditions(
    n: Int,
    memoized: Boolean,
): Int =
    if (memoized) {
        val additions = CountingAdditions()
        streamRef(additions.fibs, n).second
    } else {
        val meter = AdditionMeter()
        var produced = 2
        while (produced <= n) {
            coldFibs(produced, meter)
            produced += 1
        }
        meter.count
    }

/** One element of the counting fibs: the Fibonacci value and the
 * additions performed so far. */
private typealias FibCount = Pair<Long, Int>

/** The memoized probe: a local tie cell lets the builder's zip reference
 * the pair stream it is building, and every zip head bumps the running
 * count once. */
private class CountingAdditions {
    var additions: Int = 0

    var tied: LStream<FibCount>? = null

    val fibs: LStream<FibCount> =
        consStream(0L to 0) {
            consStream(1L to 0) {
                val fibs = checkNotNull(tied) { "counting fibs not yet tied" }
                zipStream(fibs, fibs.streamTail()) { a, b ->
                    additions += 1
                    Math.addExact(a.first, b.first) to additions
                }
            }
        }

    init {
        tied = fibs
    }
}

/** Counts the additions a cold walk performs. */
private class AdditionMeter {
    var count: Int = 0
}

/** One fresh derivation of element [i] over an unmemoized delay: the
 * zip re-derives both predecessors before adding their heads, and the
 * meter records the addition. */
private fun coldFibs(
    i: Int,
    meter: AdditionMeter,
): Long =
    when (i) {
        0 -> {
            0L
        }

        1 -> {
            1L
        }

        else -> {
            val a = coldFibs(i - 2, meter)
            val b = coldFibs(i - 1, meter)
            meter.count += 1
            Math.addExact(a, b)
        }
    }
