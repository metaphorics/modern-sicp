// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.21

package sicp.ch2.exercises

import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.renderDatum

/** Apply [f] to every whole-number element of a pair chain. */
public fun mapList(
    f: (Long) -> Long,
    items: Datum,
): Datum = if (items === Empty) Empty else pair(Whole(f(wholeNumber(firstPart(items)))), mapList(f, secondPart(items)))

/** Square each number recursively, constructing the result in the same order. */
public fun squareList(items: Datum): Datum =
    if (items === Empty) {
        Empty
    } else {
        val value = wholeNumber(firstPart(items))
        pair(Whole(value * value), squareList(secondPart(items)))
    }

/** Define the same operation through [mapList]. */
public fun squareListViaMap(items: Datum): Datum = mapList({ value -> value * value }, items)

/** Canonical native rendering of the squared sample sequence. */
public fun ex_2_21(): String = renderDatum(squareList(datumList(Whole(1L), Whole(2L), Whole(3L), Whole(4L))))
