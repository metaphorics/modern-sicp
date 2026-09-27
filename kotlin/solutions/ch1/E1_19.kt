// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.19

package sicp.ch1.exercises

import java.math.BigInteger

private fun isEven(n: Long): Boolean = n % 2L == 0L

/**
 * `T_pq` applied twice equals one `T_p'q'` with `p' = p^2 + q^2` and
 * `q' = q^2 + 2pq`: expanding `T_pq(T_pq(a, b))` and matching it against
 * `T_p'q'(a, b)` term by term gives exactly these two identities.
 */
private tailrec fun fibIterLog(
    a: BigInteger,
    b: BigInteger,
    p: BigInteger,
    q: BigInteger,
    count: Long,
): BigInteger =
    when {
        count == 0L -> {
            b
        }

        isEven(count) -> {
            val pNext = p * p + q * q
            val qNext = q * q + BigInteger.TWO * p * q
            fibIterLog(a, b, pNext, qNext, count / 2L)
        }

        else -> {
            val aNext = b * q + a * q + a * p
            val bNext = b * p + a * q
            fibIterLog(aNext, bNext, p, q, count - 1L)
        }
    }

/** Theta(log n) steps: `T^n` is assembled by repeated squaring of the transform. */
public fun fibLog(n: Long): BigInteger = fibIterLog(BigInteger.ONE, BigInteger.ZERO, BigInteger.ZERO, BigInteger.ONE, n)

public fun ex_1_19(n: Long): BigInteger = fibLog(n)
