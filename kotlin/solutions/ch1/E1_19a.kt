// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.19a

package sicp.ch1.exercises

private fun isEven(n: Long): Boolean = n % 2L == 0L

/**
 * The same halve/fold transform as [fibLog], over checked `Long`
 * arithmetic: every addition and multiplication throws
 * `ArithmeticException` rather than silently wrapping past
 * `Long.MAX_VALUE`.
 */
private tailrec fun fibIterLogChecked(
    a: Long,
    b: Long,
    p: Long,
    q: Long,
    count: Long,
): Long =
    when {
        count == 0L -> {
            b
        }

        isEven(count) -> {
            val pNext = Math.addExact(Math.multiplyExact(p, p), Math.multiplyExact(q, q))
            val qNext = Math.addExact(Math.multiplyExact(q, q), Math.multiplyExact(Math.multiplyExact(2L, p), q))
            fibIterLogChecked(a, b, pNext, qNext, count / 2L)
        }

        else -> {
            val aNext =
                Math.addExact(Math.addExact(Math.multiplyExact(b, q), Math.multiplyExact(a, q)), Math.multiplyExact(a, p))
            val bNext = Math.addExact(Math.multiplyExact(b, p), Math.multiplyExact(a, q))
            fibIterLogChecked(aNext, bNext, p, q, count - 1L)
        }
    }

public fun fibLogChecked(n: Long): Long = fibIterLogChecked(1L, 0L, 0L, 1L, n)

public fun ex_1_19a(n: Long): Long = fibLogChecked(n)
