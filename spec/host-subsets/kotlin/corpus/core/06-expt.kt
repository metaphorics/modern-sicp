// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/06-expt: successive squaring
fun isEven(n: Long): Boolean = n % 2L == 0L

fun square(x: Long): Long = x * x

fun fastExpt(b: Long, n: Long): Long =
    when {
        n == 0L -> 1L
        isEven(n) -> square(fastExpt(b, n / 2L))
        else -> b * fastExpt(b, n - 1L)
    }

fun main() {
    println(fastExpt(2L, 10L))
}
