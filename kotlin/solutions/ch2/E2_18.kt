// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.18

package sicp.ch2.exercises

import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.renderDatum

/** Reverse a finite proper pair chain by accumulating one cell at a time. */
public fun reverseList(items: Datum): Datum = reverseAcc(items, Empty)

private tailrec fun reverseAcc(
    items: Datum,
    answer: Datum,
): Datum = if (items === Empty) answer else reverseAcc(secondPart(items), pair(firstPart(items), answer))

/** Canonical native rendering of the reversed sample sequence. */
public fun ex_2_18(): String = renderDatum(reverseList(datumList(Whole(1L), Whole(4L), Whole(9L), Whole(16L), Whole(25L))))
