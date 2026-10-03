// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.37

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.37: Ben Bitdiddle's generator, which computes the
 * hypotenuse instead of choosing it. Ben is right about the search size:
 * the `hsq >= ksq` requirement prunes each (i, j) branch before any
 * third choice point is entered, and the integer-square-root test
 * replaces the k choice outright, so his generator consumes a fraction
 * of the choices the 4.35 program spends -- and both answer [3, 4, 5]
 * first. The backtrack counts below are the edition's metric: one
 * backtrack per failure resumption that delivers an alternative from a
 * pending choice frame, counted to the first answer.
 *
 * Expected answers: both generators answer [3, 4, 5] first; the 4.35
 * order costs 461 backtracks to the first triple and Ben's costs 42.
 */
public fun benFirstTriple(): String = throw PendingSolution()

public fun bookOrderBacktracksToFirst(): Long = throw PendingSolution()

public fun benBacktracksToFirst(): Long = throw PendingSolution()
