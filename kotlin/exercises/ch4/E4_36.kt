// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.36

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.36: all Pythagorean triples, unbounded. Replacing
 * `anIntegerBetween` with `anIntegerStartingFrom` in the 4.35
 * procedure strands the search on its first two choices forever: the
 * innermost unbounded choice never exhausts, so j and i never advance
 * past their first values, and [1, 1, k] is never a triple. The edition's
 * fair-interleaving rule grows the hypotenuse instead: k runs unbounded
 * and each finite k is searched out fully with i and j between 1 and k,
 * so every triple is reached after finitely many choices.
 *
 * Expected answers: the fair generator's first six triples are [3, 4, 5],
 * [6, 8, 10], [5, 12, 13], [9, 12, 15], [8, 15, 17], [12, 16, 20]; the naive
 * replacement delivers no triple before its 600-choice budget raises
 * `choice budget exhausted after 600 choices`.
 */
public fun fairTriplesFirstSix(): List<String> = throw PendingSolution()

public fun naiveBudgetFault(): String = throw PendingSolution()
