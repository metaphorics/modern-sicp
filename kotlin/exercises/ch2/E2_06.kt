// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.6

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/** A Church numeral: a function that applies its argument `n` times. Given by the exercise statement. */
public typealias Church<T> = ((T) -> T) -> (T) -> T

/** Given by the exercise statement: zero applies its function zero times, to any starting value. */
public fun <T> churchZero(): Church<T> = { _ -> { x -> x } }

/** Given by the exercise statement: one more application than [n] makes. */
public fun <T> churchAddOne(n: Church<T>): Church<T> = { f -> { x -> f(n(f)(x)) } }

public fun churchToLong(n: Church<Long>): Long = n({ x -> x + 1L })(0L)

/**
 * Exercise 2.6: in case representing pairs as procedures wasn't
 * mind-boggling enough, consider that, in a language that can manipulate
 * procedures, we can get by without numbers, by implementing 0 and the
 * operation of adding 1 as `churchZero` and `churchAddOne` above. This
 * representation is known as Church numerals, after Alonzo Church. Define
 * `churchOne` and `churchTwo` directly, not in terms of `churchZero` and
 * `churchAddOne`. Give a direct definition of the addition procedure
 * `plusChurch`, not in terms of repeated application of `churchAddOne`.
 * (Hint: use substitution to evaluate `churchAddOne(churchZero())`.) The
 * statement lives in the section 2.1 chapter text.
 *
 * The scaffold returns `churchToLong(plusChurch(churchOne(), churchTwo()))`.
 */
public fun ex_2_06(): Long = throw PendingSolution()
