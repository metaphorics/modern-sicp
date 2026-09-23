// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.5

package sicp.ch2.exercises

import java.math.BigInteger

private val two: BigInteger = BigInteger.valueOf(2L)
private val three: BigInteger = BigInteger.valueOf(3L)

public fun consPow(
    a: Long,
    b: Long,
): BigInteger = two.pow(a.toInt()) * three.pow(b.toInt())

/** Counts how many times [z] divides by 2 before an odd (or 3-only) remainder is left. */
public fun carPow(z: BigInteger): Long {
    var n = z
    var count = 0L
    while (n.mod(two) == BigInteger.ZERO) {
        n /= two
        count++
    }
    return count
}

public fun cdrPow(z: BigInteger): Long {
    var n = z
    var count = 0L
    while (n.mod(three) == BigInteger.ZERO) {
        n /= three
        count++
    }
    return count
}

public fun ex_2_05(): Pair<Long, Long> {
    val z = consPow(3L, 4L)
    return carPow(z) to cdrPow(z)
}
