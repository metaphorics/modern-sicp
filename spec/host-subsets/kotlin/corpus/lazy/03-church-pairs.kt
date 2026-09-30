// SPDX-License-Identifier: GPL-3.0-only
// Corpus case lazy/03-church-pairs: lazy pairs and delayed tails
fun main() {
    val p = lazyPair(1L, thunk {
        println("tail")
        listOf(2L, 3L)
    })
    println(p.get(0))
    println(p.get(1))
    println(p.get(2))
}
