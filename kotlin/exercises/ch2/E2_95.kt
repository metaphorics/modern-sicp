// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.95

package sicp.ch2.exercises

/**
 * Exercise 2.95: with `P1 = x^2 - 2x + 1`, `P2 = 11x^2 + 7`, and
 * `P3 = 13x + 5`, let `Q1 = P1*P2` and `Q2 = P1*P3` and compute their
 * GCD. The answer is not `P1`: `Q2`'s leading coefficient 13 does not
 * divide `Q1`'s leading coefficient 11, and this edition's integer
 * coefficient division truncates, so the first quotient term comes out
 * zero, no progress is possible, and Euclid's algorithm cannot descend.
 * [traceGcdTerms] records each step so the failure stays observable --
 * exactly the "try tracing gcd-terms" the exercise asks for.
 */
public fun ex_2_95(): List<String> = throw PendingExercise()
