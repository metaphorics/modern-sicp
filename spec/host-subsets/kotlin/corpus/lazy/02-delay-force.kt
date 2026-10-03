// SPDX-License-Identifier: GPL-3.0-only
// Corpus case lazy/02-delay-force: memoized thunk computes once
fun main() {
    val t = thunk {
        println("computing")
        21L
    }
    println(force(t))
    println(force(t))
}
