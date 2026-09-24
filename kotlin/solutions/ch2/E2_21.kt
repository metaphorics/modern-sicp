// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.21

package sicp.ch2.exercises

import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.vlist

/** The book's `map` over the chain, as the chapter text defines it. */
public fun mapList(
    f: (Long) -> Long,
    items: Value,
): Value = if (items is VNil) VNil else cons(VInt(f(numOf(carOf(items)))), mapList(f, cdrOf(items)))

/** The book's first definition: cons the square of the head onto the square-list of the rest. */
public fun squareList(items: Value): Value =
    if (items is VNil) VNil else cons(VInt(numOf(carOf(items)) * numOf(carOf(items))), squareList(cdrOf(items)))

/** The book's second definition: `mapList` with a squaring function. */
public fun squareListViaMap(items: Value): Value = mapList({ x -> x * x }, items)

/** Both definitions square `(1 2 3 4)` to `(1 4 9 16)`; the direct one, printed. */
public fun ex_2_21(): String = squareList(vlist(VInt(1L), VInt(2L), VInt(3L), VInt(4L))).toString()
