// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.17

package sicp.ch1.exercises

private fun double(x: Long): Long = x + x

private fun halve(x: Long): Long = x / 2L

private fun isEven(n: Long): Boolean = n % 2L == 0L

/**
 * The `fastExpt` shape carried over to multiplication: even `b` halves
 * (with `a` doubled to match), odd `b` peels off one addition of `a`.
 * The even branch's self-call is in tail position; the odd branch's is
 * not, wrapped in `a +`, so this function cannot be marked `tailrec`.
 */
public fun ex_1_17(
    a: Long,
    b: Long,
): Long =
    when {
        b == 0L -> 0L
        isEven(b) -> ex_1_17(double(a), halve(b))
        else -> a + ex_1_17(a, b - 1L)
    }
