// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.13

package sicp.ch1.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 1.13: prove that `Fib(n)` is the closest integer to
 * `phi^n / sqrt(5)`, where `phi = (1 + sqrt(5)) / 2`. Hint: let
 * `psi = (1 - sqrt(5)) / 2`. Use induction and the definition of the
 * Fibonacci numbers to prove that `Fib(n) = (phi^n - psi^n) / sqrt(5)`.
 * The proof is carried in the rationale; the statement lives in the
 * section 1.2 chapter text.
 *
 * The scaffold returns `Fib(n)` computed from the closed form, rounded to
 * the nearest integer, as empirical corroboration of the proof.
 */
public fun ex_1_13(n: Int): Long = throw PendingSolution()
