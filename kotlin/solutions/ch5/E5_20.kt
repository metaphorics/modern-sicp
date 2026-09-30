// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.20: the box-and-pointer and memory-vector
// representations of the two definitions of the section -- a pair of 1
// and 2 named `x`, and the list of `x` twice named `y` -- with the free
// pointer initially at p1. The three conses allocate in a forced order:
// a register machine cannot fill the outer cell before it has computed
// the inner one, so the cells come out at p1, p2, p3, x is p1, y is p3,
// and free ends at p4. Both elements of y name the same cell p1: the
// sharing the box-and-pointer drawing shows as two arrows into one box,
// and the memory-vector drawing as one pointer value written twice.

package sicp.ch5.solutions

import sicp.guest.GValue

// ---------------------------------------------------------------------------

/** The exercise run through the allocation path: the two definitions'
 *  three conses against a memory whose free pointer starts at p1, then
 *  the the-cars/the-cdrs table and the two pointer answers with the
 *  final value of free. */
public fun memoryVectorDrawing(): List<String> =
    machineScope {
        val memory = Memory(size = 8, free = 1)
        val x = memory.cons(numberWord(1), numberWord(2))
        val inner = memory.cons(x, GValue.VNull)
        val y = memory.cons(x, inner)
        memory.dump().split("\n") +
            listOf(
                "x = ${wordToString(x)}",
                "y = ${wordToString(y)}",
                "free = ${wordToString(pairPointer(memory.free))}",
            )
    }
