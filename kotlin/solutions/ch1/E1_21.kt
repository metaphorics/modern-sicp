// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.21

package sicp.ch1.exercises

private fun divides(
    a: Long,
    b: Long,
): Boolean = b % a == 0L

private tailrec fun findDivisor(
    n: Long,
    testDivisor: Long,
): Long =
    when {
        testDivisor * testDivisor > n -> n
        divides(testDivisor, n) -> testDivisor
        else -> findDivisor(n, testDivisor + 1L)
    }

public fun smallestDivisor(n: Long): Long = findDivisor(n, 2L)

public fun ex_1_21(): Triple<Long, Long, Long> = Triple(smallestDivisor(199L), smallestDivisor(1999L), smallestDivisor(19999L))
