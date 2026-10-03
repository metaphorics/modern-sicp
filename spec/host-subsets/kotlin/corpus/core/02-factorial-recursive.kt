// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/02-factorial-recursive: linear recursion
fun factorial(n: Long): Long = if (n < 2L) 1L else n * factorial(n - 1L)

fun main() {
    println(factorial(10L))
}
