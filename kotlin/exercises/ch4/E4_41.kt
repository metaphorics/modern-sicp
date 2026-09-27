// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.41

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.41: an ordinary Kotlin program (no `amb`) that solves the
 * multiple dwelling puzzle. The solver visits the assignments in the same
 * order the nondeterministic program's choices generate them -- baker
 * major, each floor 1 to 5 -- applies the puzzle's restrictions to each,
 * and answers the first one that survives.
 *
 * Expected answer: exactly `((baker 3) (cooper 2) (fletcher 4) (miller 5)
 * (smith 1))`, the same assignment the amb evaluator answers first.
 */
public fun kotlinSolverAnswer(): String = throw PendingSolution()

/**
 * The number of candidate assignments the plain solver tests before the
 * answer survives. The restrictions run only after all five floors are
 * assigned, so the solver walks the grid in generation order until the
 * survivor at grid code 1470.
 *
 * Expected answer: 1471 tests of the 3125-candidate grid.
 */
public fun kotlinSolverTests(): Int = throw PendingSolution()
