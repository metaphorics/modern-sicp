// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/07-fibonacci: tree recursion
fun fib(n: Long): Long = if (n < 2L) n else fib(n - 1L) + fib(n - 2L)

fun main() {
    println(fib(10L))
}
