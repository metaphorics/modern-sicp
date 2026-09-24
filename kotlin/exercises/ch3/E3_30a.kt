// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.30a

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.30a (added by this edition; extends exercise 3.30): check
 * the ripple-carry adder against integer addition. Build a fresh
 * `Simulation`, wire a four-bit `rippleCarryAdder` over fourteen fresh
 * wires, set the wires to the bits of `a`, `b`, and `cIn` (wire 1 is
 * the least significant bit, as in figure 3.27), run the schedule to
 * exhaustion, and answer the assembled result: the four sum wires as
 * the low four bits and the carry-out wire as the fifth bit. The check
 * is that this value equals the integer sum `a + b + cIn` for every
 * input, because a correct adder is exactly a circuit that computes
 * integer addition on its wire values.
 */
public fun rippleAdd(
    a: Int,
    b: Int,
    cIn: Int,
): Int = throw PendingSolution()
