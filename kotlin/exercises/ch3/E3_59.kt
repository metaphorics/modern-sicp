// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.59

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.59, power series as coefficient streams: the series
 * a0 + a1 x + a2 x^2 + ... is the stream a0, a1, a2, ....
 *
 * Part (a): the integral of the series is c + a0 x + (1/2) a1 x^2 +
 * (1/3) a2 x^3 + ..., so the non-constant coefficients are a0/1,
 * a1/2, a2/3, ...: return each coefficient divided by its index plus
 * one. The result has no constant term; callers cons one on.
 *
 * Part (b): e^x is its own derivative, so its series is its own
 * integral past the constant term e^0 = 1. Generate sine and cosine
 * from d(sin) = cos and d(cos) = -sin the same way, starting from the
 * constants sin 0 = 0 and cos 0 = 1. Coefficients are exact rationals.
 */
public fun integrateSeries(s: LStream<Rat>): LStream<Rat> = throw PendingSolution()

/** The series of e^x, its own integral past the constant 1. */
public val expSeries: LStream<Rat> = throw PendingSolution()

/** The series of sin x, the integral of cos x past the constant 0. */
public val sinSeries: LStream<Rat> = throw PendingSolution()

/** The series of cos x, the integral of -sin x past the constant 1. */
public val cosSeries: LStream<Rat> = throw PendingSolution()
