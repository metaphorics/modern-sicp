// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.34

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.34: Louis Reasoner builds a squarer out of one multiplier,
 * wiring `a` to both factor terminals so `b` should always hold the
 * square of `a`. The device has a serious flaw. Implement Louis's
 * device exactly as proposed; the tests demonstrate the flaw by
 * observation -- give `b` a value and watch `a` stay unknown.
 */
public fun louisSquarer(
    a: Connector,
    b: Connector,
): Constraint = throw PendingSolution()
