// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.22

package sicp.ch2.exercises

import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.vlist

private fun square(v: Value): Value = VInt(numOf(v) * numOf(v))

/**
 * Louis's first attempt: iterative, but each square is consed onto the
 * accumulated answer, so the answer chain is built back to front.
 */
public fun squareListIter(items: Value): Value {
    tailrec fun iter(
        things: Value,
        answer: Value,
    ): Value = if (things is VNil) answer else iter(cdrOf(things), cons(square(carOf(things)), answer))

    return iter(items, VNil)
}

/**
 * Louis's swapped-`cons` attempt: now the squares end up in order, but
 * each `cons` makes the accumulated answer the head and the square the
 * tail, so the result is an improper chain of nested pairs, not a list.
 */
public fun squareListIterSwapped(items: Value): Value {
    tailrec fun iter(
        things: Value,
        answer: Value,
    ): Value = if (things is VNil) answer else iter(cdrOf(things), cons(answer, square(carOf(things))))

    return iter(items, VNil)
}

/** Louis's first attempt applied to `(1 2 3 4)`, printed: `(16 9 4 1)`, in reverse order. */
public fun ex_2_22(): String = squareListIter(vlist(VInt(1L), VInt(2L), VInt(3L), VInt(4L))).toString()
