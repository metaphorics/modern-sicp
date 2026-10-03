// SPDX-License-Identifier: GPL-3.0-only
// Corpus case compiler/01-primitive-combination: open-coded primitive
// combinations and precedence under compilation (section 5.5.2).
fun main() {
    val a = 3L + 4L * 5L
    val b = (10L - 4L) * (2L + 1L)
    val c = 7L % 4L + 20L / 3L
    println(a)
    println(b)
    println(c)
}
