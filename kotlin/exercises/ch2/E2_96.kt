// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.96

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 2.96: pseudodivision. Before any polynomial division in the
 * GCD computation, multiply the dividend by the integerizing factor
 * `c^(1 + O1 - O2)`, where `c` is the divisor's leading coefficient, so
 * the division introduces no fractions; the remainder is the
 * pseudoremainder. Part (b) divides the answer's coefficients by their
 * integer GCD. The edition applies that content reduction after every
 * pseudoremainder, not only to the final answer, which keeps the `Long`
 * coefficients in range and lands on the same reduced result.
 */
public fun ex_2_96(): String = throw PendingSolution()
