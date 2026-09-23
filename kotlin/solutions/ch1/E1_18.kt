// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.18

package sicp.ch1.exercises

private fun double(x: Long): Long = x + x

private fun halve(x: Long): Long = x / 2L

private fun isEven(n: Long): Boolean = n % 2L == 0L

/** The invariant `product + a * b` never changes; `product` carries the answer at `b = 0`. */
private tailrec fun multiplyIter(
    product: Long,
    a: Long,
    b: Long,
): Long =
    when {
        b == 0L -> product
        isEven(b) -> multiplyIter(product, double(a), halve(b))
        else -> multiplyIter(product + a, a, b - 1L)
    }

public fun ex_1_18(
    a: Long,
    b: Long,
): Long = multiplyIter(0L, a, b)
