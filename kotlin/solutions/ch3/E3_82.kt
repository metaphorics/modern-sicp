// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.82

package sicp.ch3.exercises

import sicp.runtime.LStream

/**
 * The stream version of 3.5's `estimate-integral`: successive Monte
 * Carlo estimates of the area fraction of the rectangle
 * `[x1, x2) x [y1, y2)` whose points satisfy [p], one estimate per
 * experiment and no trial-count argument. Experiment points are
 * consecutive pairs of [randomNumbers] mapped into the box; each
 * estimate is the rectangle area times the running hit fraction.
 */
public fun estimateIntegral(
    p: (Double, Double) -> Boolean,
    x1: Double,
    x2: Double,
    y1: Double,
    y2: Double,
): LStream<Double> {
    val experiments: LStream<Boolean> =
        mapSuccessivePairs(
            { r1, r2 -> p(unitBoxWord(r1, x1, x2), unitBoxWord(r2, y1, y2)) },
            randomNumbers,
        )
    val estimates: LStream<Double> = monteCarloStream(experiments, 0L, 0L)
    return streamMap({ estimate -> (x2 - x1) * (y2 - y1) * estimate }, estimates)
}

/**
 * One generator word mapped into `[low, high)`: the top 53 bits are an
 * exact multiple of 1/2^53, so the unit-interval fraction is uniform and
 * exactly reproducible before the interval scaling.
 */
private fun unitBoxWord(
    word: ULong,
    low: Double,
    high: Double,
): Double = low + (word shr 11).toDouble() / 9007199254740992.0 * (high - low)
