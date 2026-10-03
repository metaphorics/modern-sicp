// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.53

package sicp.ch2.exercises

import sicp.runtime.Datum
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.Truth
import sicp.runtime.datumList
import sicp.runtime.renderDatum

private fun symbolTail(
    item: Datum,
    sequence: Datum,
): Datum =
    when (sequence) {
        is PairCell -> {
            if (sequence.first == item) {
                sequence
            } else {
                symbolTail(item, sequence.second)
            }
        }

        else -> {
            Truth(false)
        }
    }

/** Seven canonical native renderings over symbols, proper lists, and nested data. */
public fun ex_2_53(): List<String> {
    val pairs =
        datumList(
            datumList(Symbol("x1"), Symbol("x2")),
            datumList(Symbol("y1"), Symbol("y2")),
        )
    val secondOuterCell = (pairs as PairCell).second as PairCell
    val shortSequence = datumList(Symbol("a"), Symbol("short"), Symbol("list"))
    val nested =
        datumList(
            datumList(Symbol("red"), Symbol("shoes")),
            datumList(Symbol("blue"), Symbol("socks")),
        )
    val flat = datumList(Symbol("red"), Symbol("shoes"), Symbol("blue"), Symbol("socks"))
    val results: List<Datum> =
        listOf(
            datumList(Symbol("a"), Symbol("b"), Symbol("c")),
            datumList(datumList(Symbol("george"))),
            secondOuterCell,
            secondOuterCell.first,
            Truth(shortSequence is PairCell && shortSequence.first is PairCell),
            symbolTail(Symbol("red"), nested),
            symbolTail(Symbol("red"), flat),
        )

    return results.map(::renderDatum)
}
