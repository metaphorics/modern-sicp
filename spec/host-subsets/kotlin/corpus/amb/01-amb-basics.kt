// SPDX-License-Identifier: GPL-3.0-only
// Corpus case amb/01-amb-basics: choice and demand, search-state progression
fun main() {
    val x = choose(1L, 2L, 3L)
    demand(x > 1L)
    println(x)
}
