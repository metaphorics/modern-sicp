// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.54

package sicp.ch2.exercises

import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Real
import sicp.runtime.Symbol
import sicp.runtime.Tagged
import sicp.runtime.Text
import sicp.runtime.Truth
import sicp.runtime.Whole
import sicp.runtime.datumList

/** Compare finite datum trees recursively, without using pair identity as equality. */
public fun myEqual(
    first: Datum,
    second: Datum,
): Boolean {
    if (first === second) return true

    return when {
        first === Empty && second === Empty -> {
            true
        }

        first is Whole && second is Whole -> {
            first.value == second.value
        }

        first is Real && second is Real -> {
            first.value == second.value
        }

        first is Truth && second is Truth -> {
            first.value == second.value
        }

        first is Symbol && second is Symbol -> {
            first.name == second.name
        }

        first is Text && second is Text -> {
            first.value == second.value
        }

        first is PairCell && second is PairCell -> {
            myEqual(first.first, second.first) && myEqual(first.second, second.second)
        }

        first is Tagged && second is Tagged -> {
            first.tag == second.tag && myEqual(first.payload, second.payload)
        }

        else -> {
            false
        }
    }
}

/** Compare one equal-shape and one unequal-shape pair of nested values. */
public fun ex_2_54(): List<Boolean> {
    val flat = datumList(Symbol("this"), Symbol("is"), Symbol("a"), Symbol("list"))
    val sameShape = datumList(Symbol("this"), Symbol("is"), Symbol("a"), Symbol("list"))
    val differentShape = datumList(Symbol("this"), datumList(Symbol("is"), Symbol("a")), Symbol("list"))
    return listOf(myEqual(flat, sameShape), myEqual(flat, differentShape))
}
