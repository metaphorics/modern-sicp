// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.50

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.50: the `ramb` special form, which visits its alternatives
 * in the seeded xorshift's order instead of left to right. The driver's
 * seed is threaded into the evaluator, so the shuffle is deterministic
 * and the sessions reproducible; mixed into Alyssa's generator it
 * escapes the first-alternative recursion that made 4.49's sentences
 * boring.
 *
 * Expected answers: under seed 20260925, `(list (ramb 1 2 3 4 5))`
 * enumerates (3), (2), (5), (1), (4); the ramb-driven generator's first
 * sentence is (sentence (simple-noun-phrase (article a) (noun professor))
 * (verb lectures)) -- different words than 4.49's, from the same
 * grammar.
 */
public fun rambEnumeration(): List<String> = throw PendingSolution()

public fun rambGeneratedFirst(): String = throw PendingSolution()
