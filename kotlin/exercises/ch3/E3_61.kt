// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.61

package sicp.ch3.exercises

import sicp.runtime.LStream
import sicp.runtime.PendingSolution

/**
 * Exercise 3.61: 1/S for a series S = 1 + S_R whose constant term is
 * 1. The reciprocal X satisfies X = 1 - S_R X, so the constant term
 * is 1 and the tail is the negated remainder of S multiplied by X
 * itself, with mulSeries from exercise 3.60 doing the multiplication.
 */
public fun invertUnitSeries(s: LStream<Rat>): LStream<Rat> = throw PendingSolution()
