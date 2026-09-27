// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.13

package sicp.ch1.exercises

import kotlin.math.pow
import kotlin.math.roundToLong
import kotlin.math.sqrt

private val phi = (1.0 + sqrt(5.0)) / 2.0
private val psi = (1.0 - sqrt(5.0)) / 2.0

/** `(phi^n - psi^n) / sqrt(5)`, rounded to the nearest integer. */
public fun closedFormFib(n: Int): Long = ((phi.pow(n) - psi.pow(n)) / sqrt(5.0)).roundToLong()

/** The direct iterative definition, kept apart as the cross-check the proof is measured against. */
public tailrec fun fibDirect(
    n: Int,
    a: Long = 0L,
    b: Long = 1L,
): Long = if (n == 0) a else fibDirect(n - 1, b, a + b)

public fun ex_1_13(n: Int): Long = closedFormFib(n)
