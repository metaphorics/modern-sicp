// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.26

package sicp.ch2.exercises

import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.renderDatum

/** Copy the first proper datum chain onto the second. */
public fun appendList(
    list1: Datum,
    list2: Datum,
): Datum = if (list1 === Empty) list2 else pair(firstPart(list1), appendList(secondPart(list1), list2))

/** Return native renderings for concatenation, pairing, and a proper two-item list. */
public fun ex_2_26(): List<String> {
    val x = datumList(Whole(1L), Whole(2L), Whole(3L))
    val y = datumList(Whole(4L), Whole(5L), Whole(6L))
    return listOf(appendList(x, y), pair(x, y), datumList(x, y)).map(::renderDatum)
}
