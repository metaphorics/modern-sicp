// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.16

package sicp.ch1.exercises

private fun isEven(n: Long): Boolean = n % 2L == 0L

/** The invariant `a * b^n` never changes from state to state; `a` carries the answer at `n = 0`. */
private tailrec fun fastExptIter(
    a: Long,
    b: Long,
    n: Long,
): Long =
    when {
        n == 0L -> a
        isEven(n) -> fastExptIter(a, b * b, n / 2L)
        else -> fastExptIter(a * b, b, n - 1L)
    }

public fun ex_1_16(
    b: Long,
    n: Long,
): Long = fastExptIter(1L, b, n)
