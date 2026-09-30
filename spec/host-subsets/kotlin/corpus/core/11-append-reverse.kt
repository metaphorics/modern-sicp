// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/11-append-reverse: list append and reverse
fun myAppend(xs: List<Long>, ys: List<Long>): List<Long> = xs.plus(ys)

fun myReverse(xs: List<Long>): List<Long> {
    var out = emptyList<Long>()
    for (x in xs) {
        out = listOf(x).plus(out)
    }
    return out
}

fun main() {
    val r = myReverse(myAppend(listOf(1L, 2L), listOf(3L)))
    println(r.get(0))
    println(r.get(2))
}
