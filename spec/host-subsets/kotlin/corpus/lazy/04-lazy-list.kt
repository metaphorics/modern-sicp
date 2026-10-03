// SPDX-License-Identifier: GPL-3.0-only
// Corpus case lazy/04-lazy-list: lazy lists and the ten-element printer rule
fun build(n: Long): List<Long> = if (n == 0L) lazyEnd else lazyPair(n, thunk { build(n - 1L) })

fun render(xs: List<Long>): String {
    var out = "["
    var i = 0
    while (i < 10) {
        if (i >= xs.size) return out + "]"
        if (i > 0) out = out + ", "
        out = out + "${xs.get(i)}"
        i = i + 1
    }
    return out + ", ...]"
}

fun main() {
    println(render(build(12L)))
}
