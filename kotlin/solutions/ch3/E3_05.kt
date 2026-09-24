// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.5

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

/** A closure over one word of hidden state: the section's `rand`, reused here as the exercise's own generator. */
public fun makeRand(seed: ULong): () -> ULong {
    var x = seed
    return {
        x = randUpdate(x)
        x
    }
}

/** The section's `monte-carlo`, reused here since exercises compile on their own. */
public fun monteCarlo(
    trials: Int,
    experiment: () -> Boolean,
): Double {
    tailrec fun iter(
        remaining: Int,
        passed: Int,
    ): Double =
        when {
            remaining == 0 -> passed.toDouble() / trials
            experiment() -> iter(remaining - 1, passed + 1)
            else -> iter(remaining - 1, passed)
        }
    return iter(trials, 0)
}

/** The book's `random-in-range`: a value drawn uniformly from `low` to `high`, scaled from the generator's raw word. */
public fun randomInRange(
    low: Double,
    high: Double,
    next: () -> ULong,
): Double {
    val range = high - low
    return low + range * (next().toDouble() / ULong.MAX_VALUE.toDouble())
}

/** Exercise 3.5's `estimate-integral`: the fraction of random rectangle points that satisfy `p`, times the rectangle's area. */
public fun estimateIntegral(
    trials: Int,
    x1: Double,
    x2: Double,
    y1: Double,
    y2: Double,
    next: () -> ULong,
    p: (Double, Double) -> Boolean,
): Double {
    val area = (x2 - x1) * (y2 - y1)
    return area * monteCarlo(trials) { p(randomInRange(x1, x2, next), randomInRange(y1, y2, next)) }
}

/** The book's closing ask: estimate pi from `estimateIntegral` measured over a unit circle. */
public fun estimatePiViaIntegral(
    trials: Int,
    next: () -> ULong,
): Double = estimateIntegral(trials, -1.0, 1.0, -1.0, 1.0, next) { x, y -> x * x + y * y <= 1.0 }
