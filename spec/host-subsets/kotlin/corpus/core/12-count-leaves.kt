// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/12-count-leaves: nested data
fun total(xss: List<List<Long>>): Long {
    var sum = 0L
    for (xs in xss) {
        for (x in xs) {
            sum = sum + x
        }
    }
    return sum
}

fun main() {
    println(total(listOf(listOf(1L, 2L), listOf(3L))))
}
