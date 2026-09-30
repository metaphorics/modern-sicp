// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/03-factorial-iterative: linear iteration
fun factorial(n: Long): Long {
    var acc = 1L
    var i = 2L
    while (i <= n) {
        acc = acc * i
        i = i + 1L
    }
    return acc
}

fun main() {
    println(factorial(10L))
}
