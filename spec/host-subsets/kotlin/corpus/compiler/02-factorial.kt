// SPDX-License-Identifier: GPL-3.0-only
// Corpus case compiler/02-factorial: compiled calls, lexical addressing,
// and a closure over a captured binding (section 5.5.4).
fun factorial(n: Long): Long = if (n < 2L) 1L else n * factorial(n - 1L)

fun makeAdder(k: Long): (Long) -> Long = { x -> x + k }

fun main() {
    val addFive = makeAdder(5L)
    println(factorial(6L))
    println(addFive(10L))
    println(factorial(3L) + addFive(0L))
}
