// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.29

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.29: what memoization buys. `(* x x)` and `(* x x x)` are
 * strict primitive applications, so each parameter use is a demand;
 * memoized, `(square (id 10))` and `(cube (id 10))` count 1 and 2;
 * unmemoized, every demand recomputes `(id 10)`, counting 2 and 5.
 *
 * Expected answers: `100`, 1, `1000`, 2 memoized; `100`, 2, `1000`, 5
 * unmemoized.
 */
public fun memoizedCountsTranscript(): String = throw PendingSolution()

public fun unmemoizedCountsTranscript(): String = throw PendingSolution()
