// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/18-strings-display: string rendering
fun shout(s: String): String = s + "!"

fun main() {
    val name = "sicp"
    println(shout(name))
    println("length=${name.length.toLong()}")
}
