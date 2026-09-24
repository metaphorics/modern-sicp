// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.6

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

private const val XORSHIFT_MULTIPLIER: ULong = 0x2545F4914F6CDD1DUL

/** One step of the seeded xorshift64* generator; see book/ch3/3.1.texi section 3.1.2 for the shared explanation. */
private fun randUpdate(x: ULong): ULong {
    var y = x
    y = y xor (y shr 12)
    y = y xor (y shl 25)
    y = y xor (y shr 27)
    return y * XORSHIFT_MULTIPLIER
}

/**
 * The book's dispatch protocol for a resettable `rand`: `generate` is
 * `(rand 'generate)`, and `reset` is `((rand 'reset) newValue)`.
 */
public interface ResettableRand {
    public fun generate(): ULong

    public fun reset(newValue: ULong): ULong
}

/**
 * Exercise 3.6: It is useful to be able to reset a random-number
 * generator to produce a sequence starting from a given value. Design a
 * `makeResettableRand` whose result answers two requests: `generate()`
 * produces a new random word, and `reset(newValue)` resets the internal
 * state to `newValue`. By resetting the state, one can generate
 * repeatable sequences, handy when testing and debugging programs that
 * use random numbers.
 *
 * The scaffold's `generate` always returns `initial` unchanged, and
 * `reset` never actually changes what `generate` answers next.
 */
public fun makeResettableRand(initial: ULong): ResettableRand = throw PendingSolution()
