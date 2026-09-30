// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.18

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.18: the alternative scan-out strategy. Instead of 4.16's
 * text strategy -- reserve the names, then interleave assignments with
 * the body -- strategy (b) first evaluates every initializer and only
 * then assigns. Because the reserved names still hold the unassigned
 * marker while any initializer runs, strategy (b) enforces a restriction
 * the text's leaves to the programmer: an initializer may not touch a
 * fellow definition during initialization. In the delayed-value shape,
 * where one definition defers its value behind a zero-argument thunk and
 * another initializer forces it eagerly, the text strategy answers 3 --
 * the forced read lands after the definer's assignment -- while strategy
 * (b) fails the forced read at initializer time.
 *
 * Expected answers: the text strategy transcript is `3`; the alternative
 * strategy fails with `UnassignedRead`.
 */
public fun textStrategyTranscript(): String = throw PendingSolution()

/** The alternative strategy's premature read. => "UnassignedRead" */
public fun altStrategyTranscript(): String = throw PendingSolution()
