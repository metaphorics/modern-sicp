// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.30

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.30: string four full-adders into the ripple-carry adder of
 * figure 3.27. Wire the chain exactly as the figure does: the carry out
 * of each stage is the carry in of the next, the external `cIn` feeds
 * stage 1, and stage 4's carry out is `cOut`. The sum wires and the
 * carry wires are the caller's; the three carries between stages are
 * local wires named after the figure's `C_1`, `C_2`, `C_3`.
 *
 * This edition fixes the width at four bits, so the constructor takes
 * the fourteen wires of one 4-bit addition instead of the book's three
 * lists of n wires.
 */
public fun Simulation.rippleCarryAdder(
    a1: Wire,
    a2: Wire,
    a3: Wire,
    a4: Wire,
    b1: Wire,
    b2: Wire,
    b3: Wire,
    b4: Wire,
    cIn: Wire,
    s1: Wire,
    s2: Wire,
    s3: Wire,
    s4: Wire,
    cOut: Wire,
): Unit = throw PendingSolution()
