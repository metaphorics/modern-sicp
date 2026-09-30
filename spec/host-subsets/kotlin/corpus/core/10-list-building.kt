// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/10-list-building: list construction and indexing
fun main() {
    val xs = listOf(1L, 2L, 3L)
    val ys = xs.plus(listOf(4L))
    println(ys.size.toLong())
    println(ys.get(3))
    println(ys.get(0))
}
