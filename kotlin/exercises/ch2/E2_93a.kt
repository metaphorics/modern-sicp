// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.93a

package sicp.ch2.exercises

/**
 * Exercise 2.93a is added by this edition and extends exercise 2.93; the
 * map's idea is "reduce rational functions lazily". Exercise 2.97 makes
 * every rational construction pay for the GCD immediately, even
 * when the caller only stacks fractions and never inspects the result.
 * [LazyRatF] defers the reduction: the unreduced numerator and
 * denominator are kept as given, and the reduced pair is computed once,
 * on first access, by Kotlin's `by lazy` delegate -- then memoized for
 * every later reader.
 */
public fun ex_2_93a(): Triple<Int, Int, Boolean> = throw PendingExercise()
