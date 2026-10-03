// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.65a (addition)

package sicp.ch2.exercises

/**
 * Exercise 2.65a (added by this edition, extends 2.65): cross-check
 * `unionSetTree`/`intersectionSetTree` against a second, independently
 * written balanced-tree set: the JDK's own `java.util.TreeSet<Long>`.
 * Build both representations from the same generated sets, compute the
 * union and intersection each way, and check that the two agree as sets
 * (ignoring order), using a Kotest property test (`checkAll`) over many
 * generated pairs of no-duplicate `Long` sets, plus the one worked
 * example below.
 *
 * The scaffold returns the worked-example check for `{1, 3, 5, 7, 9}` and
 * `{3, 5, 7, 9, 11}`.
 */
public fun treeOf(elements: Set<Long>): SetTree = throw PendingExercise()

public fun ex_2_65a(): Boolean = throw PendingExercise()
