// SPDX-License-Identifier: GPL-3.0-only
// Corpus case core/20-error-stop: typed guest error stops the run
fun divide(a: Long, b: Long): Long = a / b

fun main() {
    println(1L)
    println(divide(1L, 0L))
    println(2L)
}
