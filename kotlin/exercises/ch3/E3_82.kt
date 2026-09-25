// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.82

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * The stream version of 3.5's `estimate-integral`: successive Monte
 * Carlo estimates of the area fraction of the rectangle
 * `[x1, x2) x [y1, y2)` whose points satisfy [p], one estimate per
 * experiment and no trial-count argument. Experiment points are
 * consecutive pairs of the prelude random stream mapped into the box;
 * each estimate is the rectangle area times the running hit fraction.
 */
public fun estimateIntegral(
    p: (Double, Double) -> Boolean,
    x1: Double,
    x2: Double,
    y1: Double,
    y2: Double,
): LStream<Double> = throw PendingSolution()
