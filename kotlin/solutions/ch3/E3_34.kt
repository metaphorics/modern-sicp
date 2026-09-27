// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.34

package sicp.ch3.exercises

/**
 * Exercise 3.34: Louis's device, exactly as proposed -- one multiplier
 * with `a` wired to both factor terminals. The flaw is in the
 * multiplier's inference order: knowing the product `b` is not enough
 * to divide out a factor when neither factor terminal has a value, so a
 * value forced on `b` leaves `a` unknown. The tests observe this.
 */
public fun louisSquarer(
    a: Connector,
    b: Connector,
): Constraint = multiplier(a, a, b)
