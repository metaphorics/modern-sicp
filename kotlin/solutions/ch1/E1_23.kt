// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.23

package sicp.ch1.exercises

private fun divides(
    a: Long,
    b: Long,
): Boolean = b % a == 0L

private fun next(testDivisor: Long): Long = if (testDivisor == 2L) 3L else testDivisor + 2L

private tailrec fun findDivisorFast(
    n: Long,
    testDivisor: Long,
): Long =
    when {
        testDivisor * testDivisor > n -> n
        divides(testDivisor, n) -> testDivisor
        else -> findDivisorFast(n, next(testDivisor))
    }

public fun smallestDivisorFast(n: Long): Long = findDivisorFast(n, 2L)

public fun ex_1_23(n: Long): Long = smallestDivisorFast(n)
