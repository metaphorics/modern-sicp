// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.26

package sicp.ch2.exercises

import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.vlist

/** The book's `append`, over the chain representation. */
public fun appendList(
    list1: Value,
    list2: Value,
): Value = if (list1 is VNil) list2 else cons(carOf(list1), appendList(cdrOf(list1), list2))

/**
 * The three evaluations, printed: `appendList` glues the two chains
 * together, `cons` makes x the single head element followed by y's
 * elements, and `vlist` makes x and y the two head elements.
 */
public fun ex_2_26(): List<String> {
    val x = vlist(VInt(1L), VInt(2L), VInt(3L))
    val y = vlist(VInt(4L), VInt(5L), VInt(6L))
    return listOf(appendList(x, y).toString(), cons(x, y).toString(), vlist(x, y).toString())
}
