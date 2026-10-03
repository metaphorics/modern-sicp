// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/15-closure-counter: lexical mutable capture shared by closures
fun main() {
    var count = 0L
    val bump: () -> Unit = { count = count + 1L }
    val read: () -> Long = { count }
    bump()
    bump()
    println(read())
}
