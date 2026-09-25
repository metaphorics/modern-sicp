// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.59

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.consStream

/**
 * Exercise 3.59, power series as coefficient streams. Part (a): the
 * integral of the series a0, a1, a2, ... has non-constant coefficients
 * a0, a1/2, a2/3, ..., so [integrateSeries] pairs each coefficient
 * with its zero-based index and divides. Part (b): e^x is its own
 * derivative, so its series is its own integral past the constant term
 * e^0 = 1; sine and cosine follow from d(sin) = cos and d(cos) = -sin.
 * All coefficients are exact rationals.
 */
public fun integrateSeries(s: LStream<Rat>): LStream<Rat> =
    zipStream(s, integersStartingFrom(0L)) { a, n -> a / Rat.of(Math.addExact(n, 1L)) }

/** The series of e^x, defined as its own integral past the constant 1. */
public val expSeries: LStream<Rat> = consStream(Rat.ONE) { integrateSeries(expSeries) }

/** The series of sin x: constant 0, then the integral of cos x. */
public val sinSeries: LStream<Rat> = consStream(Rat.ZERO) { integrateSeries(cosSeries) }

/** The series of cos x: constant 1, then the integral of -sin x. */
public val cosSeries: LStream<Rat> = consStream(Rat.ONE) { integrateSeries(streamMap({ -it }, sinSeries)) }
