// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/08-cond-branching: conditional dispatch
fun classify(x: Long): Long =
    when (x) {
        0L -> 100L
        1L -> 200L
        else -> 300L
    }

fun main() {
    println(classify(0L))
    println(classify(5L))
}
