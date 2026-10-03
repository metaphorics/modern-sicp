// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/17-dotted-pairs: pair structure and destructuring
fun main() {
    val p = 1L to 2L
    val (a, b) = p
    println(a)
    println(b)
    println(a * 10L + b)
}
