// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.25

package sicp.ch2.exercises

import sicp.runtime.Datum
import sicp.runtime.Whole
import sicp.runtime.datumList

private val listA = datumList(Whole(1L), Whole(3L), datumList(Whole(5L), Whole(7L)), Whole(9L))

private val listB = datumList(datumList(Whole(7L)))

private val listC =
    datumList(
        Whole(1L),
        datumList(
            Whole(2L),
            datumList(
                Whole(3L),
                datumList(
                    Whole(4L),
                    datumList(Whole(5L), datumList(Whole(6L), Whole(7L))),
                ),
            ),
        ),
    )

/** Repeatedly select the second cell's first value. */
private fun descendFirstAfterSecond(
    start: Datum,
    times: Int,
): Datum {
    var value = start
    repeat(times) { value = firstPart(secondPart(value)) }
    return value
}

/**
 * Apply native pair selectors to three nested datum values. Each accessor
 * chain reaches the same final whole-number leaf.
 */
public fun ex_2_25(): List<Long> =
    listOf(
        wholeNumber(firstPart(secondPart(firstPart(secondPart(secondPart(listA)))))),
        wholeNumber(firstPart(firstPart(listB))),
        wholeNumber(descendFirstAfterSecond(listC, times = 6)),
    )
