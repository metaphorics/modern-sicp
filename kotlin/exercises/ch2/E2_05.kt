// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.5

package sicp.ch2.exercises

/**
 * Exercise 2.5: show that we can represent pairs of nonnegative integers
 * using only numbers and arithmetic operations if we represent the pair `a`
 * and `b` as the integer that is the product `2^a * 3^b`. Give the
 * corresponding definitions of the procedures `consPow`, `carPow`, and
 * `cdrPow`. A `Long` overflows for large exponents, so this edition uses
 * `java.math.BigInteger` (D14, D17). The statement lives in the section 2.1
 * chapter text.
 *
 * The scaffold returns `carPow(consPow(3, 4))` paired with
 * `cdrPow(consPow(3, 4))`.
 */
public fun ex_2_05(): Pair<Long, Long> = throw PendingExercise()
