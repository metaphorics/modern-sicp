// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.18

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.18: the alternative scan-out strategy. Instead of 4.16's text
 * strategy -- reserve the names, then interleave `set!` with the body --
 * strategy (b) first evaluates every initializer and only then assigns.
 * Because the reserved names still hold `*unassigned*` while any
 * initializer runs, strategy (b) enforces a restriction the text's leaves
 * to the programmer: an initializer may not touch a fellow define during
 * initialization. In the 3.5.4 shape, where one define defers its value
 * behind a zero-argument lambda and another initializer forces it eagerly,
 * the text strategy answers 3 -- the forced read lands after the definer's
 * `set!` -- while strategy (b) fails the forced read at initializer time.
 *
 * Expected answers: the text strategy transcript is `3`; the alternative
 * strategy fails with `Error: type mismatch: dy is read before it is
 * assigned`.
 */
public fun textStrategyTranscript(): String = throw PendingSolution()

public fun altStrategyTranscript(): String = throw PendingSolution()
