// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.38

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.38: the multiple dwelling puzzle with the Smith-Fletcher
 * adjacency requirement omitted. The modified puzzle has five solutions,
 * enumerated here in the search's own order; the book's answer
 * [[baker, 3], [cooper, 2], [fletcher, 4], [miller, 5], [smith, 1]] is the fourth.
 *
 * Expected answer: ((baker 1) (cooper 2) (fletcher 4) (miller 3)
 * (smith 5)), [[baker, 1], [cooper, 2], [fletcher, 4], [miller, 5], [smith, 3]],
 * [[baker, 1], [cooper, 4], [fletcher, 2], [miller, 5], [smith, 3]], ((baker 3)
 * (cooper 2) (fletcher 4) (miller 5) (smith 1)), ((baker 3) (cooper 4)
 * (fletcher 2) (miller 5) (smith 1)); the independent brute force in the
 * test enumerates the same five in the same order.
 */
public fun modifiedDwellingSolutions(): List<String> = throw PendingSolution()

public fun modifiedDwellingBruteForce(): List<String> = throw PendingSolution()
