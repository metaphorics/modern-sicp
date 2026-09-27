// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.18

package sicp.ch2.exercises

import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.vlist

/** The book's `reverse`: conses each head onto the reversed rest, iteratively. */
public fun reverseList(items: Value): Value = reverseAcc(items, VNil)

private tailrec fun reverseAcc(
    items: Value,
    answer: Value,
): Value = if (items is VNil) answer else reverseAcc(cdrOf(items), cons(carOf(items), answer))

/** `reverseList` of the book's `(1 4 9 16 25)`, printed. */
public fun ex_2_18(): String = reverseList(vlist(VInt(1L), VInt(4L), VInt(9L), VInt(16L), VInt(25L))).toString()
