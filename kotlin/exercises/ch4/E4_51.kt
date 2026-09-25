// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.51

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.51: `permanent-set!`, which is not undone on failure.
 * The counting session's transcripts under both assignment forms pin
 * the difference: with `permanent-set!` every trial accumulates, and
 * the counter the driver reads back keeps the total; with `set!` every
 * failed trial is rolled back, each answer shows the same single count
 * for its own branch, and the counter reads 0 once the problem ends.
 *
 * Expected answers: `permanent-set!` answers (a b 2), (a c 3), (b a 4)
 * and the counter reads 4 afterward; `set!` answers (a b 1), (a c 1),
 * (b a 1) and the counter reads 0.
 */
public fun permanentSetTranscript(): String = throw PendingSolution()

public fun setBangTranscript(): String = throw PendingSolution()
