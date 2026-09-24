// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.37

package sicp.ch3.exercises

/** The book's `c+`: a fresh connector constrained to x + y. */
public fun cPlus(
    x: Connector,
    y: Connector,
): Connector {
    val z = Connector()
    adder(x, y, z)
    return z
}

/** The book's `c-`: a fresh connector constrained to x - y. */
public fun cMinus(
    x: Connector,
    y: Connector,
): Connector {
    val z = Connector()
    adder(z, y, x)
    return z
}

/** The book's `c*`: a fresh connector constrained to the product. */
public fun cMul(
    x: Connector,
    y: Connector,
): Connector {
    val z = Connector()
    multiplier(x, y, z)
    return z
}

/**
 * The book's `c/`: a fresh connector constrained to the quotient, by
 * wiring y * z = x. Quotients are Long division: exact only when x
 * divides by y, so networks like the converter below order their
 * multiplications to keep every division exact.
 */
public fun cDiv(
    x: Connector,
    y: Connector,
): Connector {
    val z = Connector()
    multiplier(y, z, x)
    return z
}

/** The book's `cv` (constant value): a connector permanently set to v. */
public fun cConst(v: Long): Connector {
    val z = Connector()
    constant(v, z)
    return z
}

/**
 * The expression-style temperature converter: cPlus(cDiv(cMul(9, x), 5),
 * 32). Multiplying x by 9 before dividing by 5 keeps the one division
 * exact; the book's literal shape (9/5) * x would truncate 9/5 to 1 and
 * turn the book's session 25 -> 77 into 25 -> 57.
 */
public fun celsiusFahrenheit(x: Connector): Connector = cPlus(cDiv(cMul(cConst(9L), x), cConst(5L)), cConst(32L))
