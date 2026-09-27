// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.53

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream

/**
 * Exercise 3.53: without running the program, describe the elements of
 * the stream defined as 1 followed by the elementwise sum of the stream
 * with itself. The book's add-streams pairs element k of s with element
 * k of s, so every element doubles its predecessor: the powers of two.
 * The edition's builder is a function, and Kotlin locals cannot appear
 * in their own initializer, so the tail thunk reads the cell that the
 * last line fills before anything can force a tail; each call then
 * returns its own memoized doubling stream, the book's single s.
 */
public fun doubledStream(): LStream<Long> {
    var tied: LStream<Long>? = null
    val built: LStream<Long> =
        consStream(1L) {
            val s = checkNotNull(tied) { "doubled stream not yet tied" }
            zipStream(s, s) { a, b -> Math.addExact(a, b) }
        }
    tied = built
    return built
}
