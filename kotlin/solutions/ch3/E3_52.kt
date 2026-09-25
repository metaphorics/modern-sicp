// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.52

package sicp.ch3.exercises

import sicp.runtime.LStream

/**
 * The probe of exercise 3.52: [accum] folds its argument into [sum] and
 * returns the new total, recording one event per call so the test can
 * see when the lazy pipeline ran it.
 */
public class AccumProbe {
    public var sum: Long = 0

    /** One entry per accum call: the argument and the total it left. */
    public val events: MutableList<String> = mutableListOf()

    /** The book's accum: sum becomes x + sum and the new sum is the
     * value the mapped stream carries. */
    public fun accum(x: Long): Long {
        sum = Math.addExact(sum, x)
        events.add("accum $x -> $sum")
        return sum
    }
}

/**
 * The counterfactual run of the book's script over a delay without the
 * memoized tail: [y] is everything the even filter ever produces and
 * [z] is the five filter's output, both different from the memoized
 * values because every force re-runs accum over the cells it scans;
 * [finalSum] is sum at the end of the script.
 */
public class ColdAccumRun(
    public val y: List<Long>,
    public val z: List<Long>,
    public val finalSum: Long,
)

/**
 * Exercise 3.52: the book's script, whose expressions are
 *
 * - `seq`, the accum map over the interval 1 to 20,
 * - `y`, the even filter of `seq`,
 * - `z`, the filter of `seq` leaving multiples of 5,
 * - `streamRef(y, 7)` and the display of `z`.
 *
 * Every [accum] call changes [AccumProbe.sum], so the value of sum after
 * each step records when the delayed evaluation ran. The cold run at the
 * end answers the last question: with a plain lambda delay each tail
 * force re-derives its cells, the filters re-run accum over the cells
 * they scan, and the responses change.
 */
public fun coldAccumScript(probe: AccumProbe): ColdAccumRun {
    // define seq: the map's first cell runs accum(1) at definition time.
    probe.accum(1L)
    // define y: the filter scans seq until the first even cell, forcing
    // and running accum(2), accum(3).
    var yCursor = 2L
    var yHead = 0L
    while (true) {
        val v = probe.accum(yCursor)
        if (v % 2 == 0L) {
            yHead = v
            break
        }
        yCursor += 1L
    }
    // define z: the filter re-derives seq from the start, re-running
    // accum over the cells it scans until the first multiple of 5.
    var zCursor = 2L
    var zHead = 0L
    while (true) {
        val v = probe.accum(zCursor)
        if (v % 5 == 0L) {
            zHead = v
            break
        }
        zCursor += 1L
    }
    // stream-ref y 7: each tail force re-derives the cells after the
    // filter's cursor, so the walk consumes the rest of the interval and
    // collects a different even stream than the memoized run did.
    val y = mutableListOf(yHead)
    var scan = yCursor + 1L
    while (scan <= 20L) {
        val v = probe.accum(scan)
        if (v % 2 == 0L) {
            y.add(v)
        }
        scan += 1L
    }
    // display-stream z: the five filter re-derives the cells after its
    // own cursor to the end of the interval.
    val z = mutableListOf(zHead)
    scan = zCursor + 1L
    while (scan <= 20L) {
        val v = probe.accum(scan)
        if (v % 5 == 0L) {
            z.add(v)
        }
        scan += 1L
    }
    return ColdAccumRun(y, z, probe.sum)
}
