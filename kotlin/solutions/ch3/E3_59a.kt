// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.59a

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.streamTail

/**
 * The section's derivative restatement for exercise 3.59: the
 * derivative of a0 + a1 x + a2 x^2 + ... has coefficients a1, 2 a2,
 * 3 a3, ..., so it is the tail of the series with each coefficient
 * multiplied by its new one-based index. With exact rationals this is
 * the exact inverse of [integrateSeries] past the constant term.
 */
public fun differentiateSeries(s: LStream<Rat>): LStream<Rat> = zipStream(s.streamTail(), integers) { a, k -> a * Rat.of(k) }
