// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.37

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.37: the expression-oriented constraint combinators, the
 * book's `c+`, `c-`, `c*`, `c/`, and `cv`. Each takes connectors (or a
 * constant) and returns the one fresh connector constrained to the
 * result, so compound constraints read as expressions.
 */
public fun cPlus(
    x: Connector,
    y: Connector,
): Connector = throw PendingSolution()

/** The book's `c-`: a connector constrained to the difference x - y. */
public fun cMinus(
    x: Connector,
    y: Connector,
): Connector = throw PendingSolution()

/** The book's `c*`: a connector constrained to the product of x and y. */
public fun cMul(
    x: Connector,
    y: Connector,
): Connector = throw PendingSolution()

/** The book's `c/`: a connector constrained to the quotient x / y. */
public fun cDiv(
    x: Connector,
    y: Connector,
): Connector = throw PendingSolution()

/** The book's `cv` (constant value): a connector permanently set to v. */
public fun cConst(v: Long): Connector = throw PendingSolution()

/**
 * The exercise's expression-style temperature converter, built from the
 * combinators above. Compute 9 times x before dividing by 5: values are
 * `Long`, and the reversed arrangement would truncate 9/5 to 1.
 */
public fun celsiusFahrenheit(x: Connector): Connector = throw PendingSolution()
