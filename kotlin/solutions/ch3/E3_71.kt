// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.71

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream

/**
 * Exercise 3.71: Ramanujan numbers are expressible as a sum of two cubes
 * in more than one way. Generate [weightedPairs] over the integers with
 * the cube-sum weight, then scan for runs of consecutive pairs at the
 * same weight: [mergeWeighted] keeps equal weights adjacent, so every
 * way of writing the number shows up inside one run. The first is
 * 1729 = 1^3 + 12^3 = 9^3 + 10^3, then 4104, 13832, 20683, 32832, and
 * 39312.
 */
public fun ramanujanNumbers(): LStream<Long> =
    streamMap({ run -> cubeWeight(run.first()) }, equalWeightRuns(weightedPairs(integers, integers, ::cubeWeight), ::cubeWeight, 2))

private fun cubeWeight(p: Pair<Long, Long>): Long =
    Math.addExact(
        Math.multiplyExact(p.first, Math.multiplyExact(p.first, p.first)),
        Math.multiplyExact(p.second, Math.multiplyExact(p.second, p.second)),
    )

/** Groups [s] into maximal runs of consecutive equal [weight] and keeps
 * the runs of at least [minRun] pairs. The scan to the next qualifying
 * run is a cursor loop, so rare hits never deepen the stack. */
private fun equalWeightRuns(
    s: LStream<Pair<Long, Long>>,
    weight: (Pair<Long, Long>) -> Long,
    minRun: Int,
): LStream<List<Pair<Long, Long>>> {
    var cursor = s
    while (cursor is LStream.Cons) {
        val w = weight(cursor.head)
        val run = mutableListOf(cursor.head)
        var rest = cursor.tail
        while (rest is LStream.Cons && weight(rest.head) == w) {
            run.add(rest.head)
            rest = rest.tail
        }
        if (run.size >= minRun) {
            val kept = run.toList()
            return consStream(kept) { equalWeightRuns(rest, weight, minRun) }
        }
        cursor = rest
    }
    return LStream.Empty
}
