// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
// Chapter 4, exercise 4.1

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.1: operand evaluation order. The host fixes its order --
 * operands evaluate left to right -- so the premise moves to the kernel's
 * own code: `listOfValues` is the explicit-recursion vehicle this exercise
 * rebuilds in both directions over an operand-recording program. The two
 * transcripts pin the same operand values in opposite orders: the probe's
 * two operands are 1 and 2, and the recorded order reads `[1, 2]` left to
 * right and `[2, 1]` right to left.
 */
public fun leftToRightTranscript(): String = throw PendingSolution()

/** The right-to-left `list-of-values` of the exercise, run on the probe. */
public fun rightToLeftTranscript(): String = throw PendingSolution()
