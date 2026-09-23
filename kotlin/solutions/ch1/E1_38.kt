// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.38

package sicp.ch1.exercises

// contFracIterative is exercise 1.37's public procedure, reused here from the same package.

/** Euler's `D_i`: 2, 1, 1, 4, 1, 1, 6, ... at every third index (1-based), 1 everywhere else. */
public fun eulerDenominator(i: Long): Double = if (i % 3L == 2L) 2.0 * ((i + 1L) / 3L) else 1.0

public fun ex_1_38(): Double = 2.0 + contFracIterative({ 1.0 }, ::eulerDenominator, 20L)
