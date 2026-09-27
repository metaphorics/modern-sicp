// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.60

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.60: with series added by coefficient-wise addition, the
 * product of s1 = c1 + S1' and s2 = c2 + S2' is c1 c2 + c1 S2' +
 * S1' (c2 + S2'), so the constant term is the product of the constant
 * terms and the tail adds the scaled tail to a product of streams.
 * Test by verifying sin^2 x + cos^2 x = 1 with the 3.59 series.
 */
public fun mulSeries(
    s1: LStream<Rat>,
    s2: LStream<Rat>,
): LStream<Rat> = throw PendingSolution()
