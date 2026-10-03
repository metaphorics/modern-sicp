// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/14-higher-order: procedures as arguments
fun inc(x: Long): Long = x + 1L

fun twice(f: (Long) -> Long, x: Long): Long = f(f(x))

fun main() {
    println(twice(::inc, 5L))
}
