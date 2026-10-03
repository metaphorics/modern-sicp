// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.76: conjunction as a merge of compatible frames.

package sicp.ch4.solutions

import sicp.ch4.QAnd
import sicp.ch4.QFrame
import sicp.ch4.QPattern
import sicp.ch4.QueryDriver
import sicp.ch4.renderAnswer

// Exercise 4.76: `and` as a merge of compatible frames. Each conjunct
// runs separately over its own stream; then every pair merges, one
// compatibility check per shared variable. Unshared bindings join;
// a variable bound twice to different terms rejects the pair. The
// probe merges the programmer frames with the supervision frames,
// counts the sixteen examinations, keeps the two compatible pairs,
// and checks the merged listing agrees with the engine's own
// conjunction as sets.

// Exercise 4.76: merge compatible frames; count the examinations.

/** One pairwise merge: shared bindings must agree, or nothing merges. */
internal fun mergeFrames(
    first: QFrame,
    second: QFrame,
): QFrame? {
    val merged = first.bindings.toMutableMap()
    for ((variable, value) in second.bindings) {
        val current = merged[variable]
        if (current == null) {
            merged[variable] = value
        } else if (current != value) {
            return null
        }
    }
    return QFrame(merged)
}

/** The merge procedure against the engine conjunction. */
public fun mergeAndDemos(): List<String> {
    val db = microshaftSystem()
    val driver = QueryDriver.streaming(db)
    val programmers =
        driver
            .run(
                QPattern(list(sym("job"), v("x"), list(sym("computer"), sym("programmer")))),
                listOf(v("x")),
            ).toList()
    val supervised =
        driver
            .run(
                QPattern(list(sym("supervisor"), v("x"), v("boss"))),
                listOf(v("x"), v("boss")),
            ).toList()
    var checks = 0L
    val merged = mutableListOf<QFrame>()
    for (left in programmers) {
        for (right in supervised) {
            checks +=
                left.bindings.keys
                    .intersect(right.bindings.keys)
                    .size
                    .toLong()
            val both = mergeFrames(left, right)
            if (both != null) {
                merged.add(both)
            }
        }
    }
    val xb = listOf(v("x"), v("boss"))
    val mergedLines = merged.flatMap { frame -> renderAnswer(frame, xb) }.toList()
    val andLines =
        answerLines(
            driver,
            QAnd(
                listOf(
                    QPattern(list(sym("job"), v("x"), list(sym("computer"), sym("programmer")))),
                    QPattern(list(sym("supervisor"), v("x"), v("boss"))),
                ),
            ),
            xb,
        )
    return mergedLines +
        listOf("merge agrees with and: ${mergedLines.toSet() == andLines.toSet()}") +
        listOf("compatibility checks: $checks")
}
