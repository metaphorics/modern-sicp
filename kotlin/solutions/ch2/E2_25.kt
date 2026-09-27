// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.25

package sicp.ch2.exercises

import sicp.runtime.VInt
import sicp.runtime.Value
import sicp.runtime.vlist

private val listA = vlist(VInt(1L), VInt(3L), vlist(VInt(5L), VInt(7L)), VInt(9L))

private val listB = vlist(vlist(VInt(7L)))

private val listC =
    vlist(
        VInt(1L),
        vlist(VInt(2L), vlist(VInt(3L), vlist(VInt(4L), vlist(VInt(5L), vlist(VInt(6L), VInt(7L)))))),
    )

/** Descends `car(cdr(...))` [times] times: each level of [listC] is its own two-element list. */
private fun descendCarCdr(
    start: Value,
    times: Int,
): Value {
    var v = start
    repeat(times) { v = carOf(cdrOf(v)) }
    return v
}

/**
 * Accessor chains that pick 7 from each of the book's lists: two cdrs
 * and two cars for `(1 3 (5 7) 9)`, two cars for `((7))`, and six
 * alternating cdr-then-car steps for the deeply nested chain.
 */
public fun ex_2_25(): List<Long> =
    listOf(
        numOf(carOf(cdrOf(carOf(cdrOf(cdrOf(listA)))))),
        numOf(carOf(carOf(listB))),
        numOf(descendCarCdr(listC, times = 6)),
    )
