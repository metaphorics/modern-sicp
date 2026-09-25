// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.62

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.62: division of power series with mulSeries (3.60) and
 * invertUnitSeries (3.61). Div-series works for any two series whose
 * denominator begins with a nonzero constant term and signals an
 * error otherwise. Use it with the 3.59 series to generate tangent:
 * tan x = sin x / cos x.
 */
public fun divSeries(
    s1: LStream<Rat>,
    s2: LStream<Rat>,
): LStream<Rat> = throw PendingSolution()

/** The series of tan x: sin x divided by cos x. */
public val tangentSeries: LStream<Rat> = throw PendingSolution()
