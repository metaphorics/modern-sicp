// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/19-deep-recursion: deep recursion depth
fun sumTo(n: Long): Long = if (n == 0L) 0L else n + sumTo(n - 1L)

fun main() {
    println(sumTo(1000L))
}
