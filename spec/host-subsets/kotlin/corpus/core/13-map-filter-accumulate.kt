// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/13-map-filter-accumulate: map, filter, fold
fun main() {
    val xs = listOf(1L, 2L, 3L, 4L)
    val doubled = xs.map { x -> x * 2L }
    val evens = doubled.filter { x -> x % 2L == 0L }
    val total = evens.fold(0L) { acc, x -> acc + x }
    println(total)
}
