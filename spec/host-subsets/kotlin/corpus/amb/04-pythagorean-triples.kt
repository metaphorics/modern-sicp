// SPDX-License-Identifier: GPL-3.0-only
// Corpus case amb/04-pythagorean-triples: Pythagorean triples by bounded search
fun main() {
    val a = choose(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L, 9L, 10L, 11L, 12L)
    val b = choose(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L, 9L, 10L, 11L, 12L)
    val c = choose(1L, 2L, 3L, 4L, 5L, 6L, 7L, 8L, 9L, 10L, 11L, 12L)
    demand(a <= b)
    demand(a * a + b * b == c * c)
    println(a * 100L + b * 10L + c)
}
