// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.72

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream

/**
 * Exercise 3.72: in the manner of 3.71, the numbers expressible as a sum
 * of two squares in three different ways, each emitted with the pairs
 * that show it. [weightedPairs] over the integers with the square-sum
 * weight, then the runs of three or more consecutive equal weights. The
 * first is 325 = 1^2 + 18^2 = 6^2 + 17^2 = 10^2 + 15^2; the next five
 * are 425, 650, 725, 845, and 850.
 */
public fun threeSquareNumbers(): LStream<Pair<Long, List<Pair<Long, Long>>>> =
    streamMap(
        { run -> squareWeight(run.first()) to run },
        equalWeightRuns(weightedPairs(integers, integers, ::squareWeight), ::squareWeight, 3),
    )

private fun squareWeight(p: Pair<Long, Long>): Long =
    Math.addExact(Math.multiplyExact(p.first, p.first), Math.multiplyExact(p.second, p.second))

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
