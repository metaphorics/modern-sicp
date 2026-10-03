// SPDX-License-Identifier: GPL-3.0-only
// Corpus case amb/02-prime-sum-pair: pairs whose sum is prime
fun isPrime(n: Long): Boolean {
    if (n < 2L) return false
    var d = 2L
    while (d * d <= n) {
        if (n % d == 0L) return false
        d = d + 1L
    }
    return true
}

fun main() {
    val a = choose(1L, 2L, 3L, 4L)
    val b = choose(1L, 2L, 3L, 4L)
    demand(isPrime(a + b))
    println(a * 10L + b)
}
