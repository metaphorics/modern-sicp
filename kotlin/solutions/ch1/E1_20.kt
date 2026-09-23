// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.20

package sicp.ch1.exercises

/** Counts every call `gcd` makes to `remainder` while eagerly reducing `gcd(206, 40)`. */
public fun ex_1_20(): Int {
    var remainderCalls = 0

    fun remainder(
        a: Long,
        b: Long,
    ): Long {
        remainderCalls += 1
        return a % b
    }

    fun gcd(
        a: Long,
        b: Long,
    ): Long = if (b == 0L) a else gcd(b, remainder(a, b))
    gcd(206L, 40L)
    return remainderCalls
}
