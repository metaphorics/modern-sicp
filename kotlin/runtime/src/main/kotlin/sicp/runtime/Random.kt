// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.Either

/**
 * The seeded xorshift64* generator of decision 0001: the book's `random` is
 * this generator, so the stochastic sections stay comparable across editions.
 */
public class Random private constructor(private var state: ULong) {
    /**
     * Returns the next unsigned 64-bit word: the xorshift steps update the
     * state, the state keeps the stepped word, and the returned word is the
     * state times the xorshift64* multiplier.
     */
    public fun next(): ULong {
        var x = state
        x = x xor (x shr 12)
        x = x xor (x shl 25)
        x = x xor (x shr 27)
        state = x
        return x * 0x2545F4914F6CDD1DUL
    }

    /**
     * Returns the next value in `0 until n` for a positive [n], the way the
     * book's `random` is used.
     */
    public fun random(n: Long): Long {
        require(n > 0) { "random(n) needs a positive n" }
        return (next() % n.toULong()).toLong()
    }

    public companion object {
        /**
         * Builds a generator from a nonzero [seed]; a zero seed is rejected
         * with [RandomError.InvalidSeed].
         */
        public fun seeded(seed: ULong): Either<RandomError, Random> =
            if (seed == 0UL) {
                Either.Left(RandomError.InvalidSeed)
            } else {
                Either.Right(Random(seed))
            }
    }
}
