// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/05-gcd: Euclidean algorithm
fun gcd(a: Long, b: Long): Long = if (b == 0L) a else gcd(b, a % b)

fun main() {
    println(gcd(206L, 40L))
}
