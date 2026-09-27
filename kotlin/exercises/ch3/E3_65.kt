// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.65

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.65: compute ln 2 = 1 - 1/2 + 1/3 - 1/4 + ... as three
 * sequences of approximations, the way the section did for pi, and
 * compare how rapidly they converge: the raw partial sums, the
 * Euler-transformed series, and the accelerated sequence of the full
 * tableau of transforms.
 */
public val ln2Raw: LStream<Double> = throw PendingSolution()

/** The Euler transform of the raw partial sums. */
public val ln2Euler: LStream<Double> = throw PendingSolution()

/** The first element of every row of the tableau of Euler transforms. */
public val ln2Accelerated: LStream<Double> = throw PendingSolution()
