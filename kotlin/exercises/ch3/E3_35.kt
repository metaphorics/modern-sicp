// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.35

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.35: Ben's squarer as a new primitive constraint. When `b`
 * holds a non-negative value, `a` becomes its square root; when `b`
 * holds a negative value the constraint fails the way the book's
 * `error` call does; when `b` has no value but `a` does, `b` becomes the
 * square. Forgetting retracts both sides and re-derives what it can.
 */
public fun squarer(
    a: Connector,
    b: Connector,
): Constraint = throw PendingSolution()
