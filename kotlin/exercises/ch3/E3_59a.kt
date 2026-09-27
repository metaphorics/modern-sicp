// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.59a

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * The section's derivative restatement: the derivative of a0 + a1 x +
 * a2 x^2 + ... has coefficients a1, 2 a2, 3 a3, ..., so return the
 * tail of the series with each coefficient multiplied by its new
 * one-based index. With exact rationals this is the exact inverse of
 * [integrateSeries] once the integral's constant term is consed on.
 */
public fun differentiateSeries(s: LStream<Rat>): LStream<Rat> = throw PendingSolution()
