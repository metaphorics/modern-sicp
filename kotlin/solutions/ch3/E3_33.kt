// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.33

package sicp.ch3.exercises

/**
 * Exercise 3.33: the averager as a three-constraint network. `adder`
 * pins `u` to a + b, the `constant` box pins `v` to 2, and the
 * `multiplier` pins `u` to c * v, so `c` is (a + b) / 2 and each of the
 * three boxes can re-derive its share from the other two. The division
 * happens in the multiplier's backward branch, so the network halves
 * exactly when a + b is even; an odd sum truncates (see the tests).
 */
public fun averager(
    a: Connector,
    b: Connector,
    c: Connector,
) {
    val u = Connector()
    val v = Connector()
    adder(a, b, u)
    multiplier(c, v, u)
    constant(2L, v)
}
