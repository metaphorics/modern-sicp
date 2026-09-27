// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.40

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.40: the assignment space before and after `distinct?`, and
 * a pruned generator that imposes each restriction as soon as its people
 * are placed. The counting metric is 4.39's: one backtrack per failure
 * resumption that delivers an alternative, counted to the first answer.
 *
 * Expected answers: 3125 assignments before the distinctness requirement
 * and 120 after; the unprompted program costs 1470 backtracks to the
 * first answer while the pruned nest costs 210, and both answer
 * ((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1)).
 */
public fun assignmentsBeforeDistinct(): Int = throw PendingSolution()

public fun assignmentsAfterDistinct(): Int = throw PendingSolution()

public fun naiveBacktracksToFirst(): Long = throw PendingSolution()

public fun prunedBacktracksToFirst(): Long = throw PendingSolution()

public fun prunedAnswer(): String = throw PendingSolution()
