// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.1

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.1: the metacircular evaluator's operand evaluation order.
 * Kotlin's own application order is fixed -- operands evaluate left to
 * right -- so the base evaluator already answers the order question the
 * Scheme text leaves open. The exercise still matters: writing the
 * operand list explicitly makes the order a property of the evaluator's
 * code rather than of the host, and the right-to-left version shows just
 * what the host had been deciding. The probe is
 * `(cons (note 1) (note 2))` with `note` pushing onto a list: the two
 * transcript pins are `"(1 . 2)\n(2 1)\n"` left to right and
 * `"(1 . 2)\n(1 2)\n"` right to left -- same values, opposite order.
 */
public fun leftToRightTranscript(): String = throw PendingSolution()

/** The right-to-left `list-of-values` of the exercise, run on the probe. */
public fun rightToLeftTranscript(): String = throw PendingSolution()
