// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.6

package sicp.ch3.exercises

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

/** A captured `var x`, advanced by `generate` and overwritten directly by `reset`. */
public fun makeResettableRand(initial: ULong): ResettableRand {
    var x = initial
    return object : ResettableRand {
        override fun generate(): ULong {
            x = randUpdate(x)
            return x
        }

        override fun reset(newValue: ULong): ULong {
            x = newValue
            return x
        }
    }
}
