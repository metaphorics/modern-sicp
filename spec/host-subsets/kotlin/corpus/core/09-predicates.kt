// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/09-predicates: Boolean-only conditions
fun isEven(n: Long): Boolean = n % 2L == 0L

fun isPositive(n: Long): Boolean = n > 0L

fun main() {
    println(isEven(4L))
    println(isEven(5L))
    println(isPositive(3L))
    println(isPositive(0L - 1L))
}
