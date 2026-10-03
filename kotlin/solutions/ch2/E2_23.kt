// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.23

package sicp.ch2.exercises

import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.Whole
import sicp.runtime.datumList

/** Apply [action] to each pair-chain element in order. */
public fun forEachValue(
    items: Datum,
    action: (Datum) -> Unit,
) {
    if (items !== Empty) {
        action(firstPart(items))
        forEachValue(secondPart(items), action)
    }
}

/** Values collected by a recording action over the sample sequence. */
public fun ex_2_23(): List<Long> {
    val seen = mutableListOf<Long>()
    forEachValue(datumList(Whole(57L), Whole(321L), Whole(88L))) { value -> seen.add(wholeNumber(value)) }
    return seen
}
