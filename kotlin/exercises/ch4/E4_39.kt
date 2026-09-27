// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.39

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.39: does the order of the restrictions matter? Both orders
 * answer the same assignment, and -- because every restriction is checked
 * only after all five floors have been chosen, so an assignment fails at
 * the same innermost choice point whichever requirement rejects it first
 * -- this engine measures identical work to the first answer under both
 * orders. Reordering helps only when a requirement moves ahead of a
 * choice, which is 4.40's pruning. The counts are the edition's metric:
 * one backtrack per failure resumption that delivers an alternative.
 *
 * Expected answers: the answer is ((baker 3) (cooper 2) (fletcher 4)
 * (miller 5) (smith 1)) under both orders; the book order and the
 * Fletcher-first reorder each cost 1470 backtracks to the first answer,
 * while 4.40's pruned program costs 210.
 */
public fun dwellingAnswer(): String = throw PendingSolution()

public fun bookOrderBacktracks(): Long = throw PendingSolution()

public fun reorderedBacktracks(): Long = throw PendingSolution()
