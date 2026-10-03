// SPDX-License-Identifier: GPL-3.0-only
// Corpus case lazy/01-non-strict-application: strict and delayed parameters
fun note(x: Long): Long {
    println(x)
    return x
}

fun unless(condition: Boolean, @Strict usualValue: Long, @Delayed exceptionalValue: Long): Long =
    if (condition) usualValue else exceptionalValue

fun main() {
    println(unless(true, note(1L), note(2L)))
    println(unless(false, note(3L), note(4L)))
}
