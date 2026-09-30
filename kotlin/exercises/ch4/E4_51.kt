// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.51

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.51: `setPermanent`, whose writes are not undone on failure.
 * The counting session's transcripts under both assignment forms pin the
 * difference: with `setPermanent` every trial accumulates, so the count
 * each answer reports keeps the total; with an ordinary assignment every
 * failed trial is rolled back, so each answer reports the same single
 * count for its own branch.
 *
 * Expected answers: `setPermanent` answers `[a, b, 2]`, `[a, c, 3]`,
 * `[b, a, 4]`; the ordinary assignment answers `[a, b, 1]`, `[a, c, 1]`,
 * `[b, a, 1]`.
 */
public fun permanentSetTranscript(): String = throw PendingSolution()

/** The same session with ordinary assignment, rolled back per branch. */
public fun ordinarySetTranscript(): String = throw PendingSolution()
