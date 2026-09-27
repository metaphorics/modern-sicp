// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.2

package sicp.ch1.exercises

/** The four primitives of the statement: every operator is a call. */
private fun sum(
    a: Double,
    b: Double,
): Double = a + b

private fun diff(
    a: Double,
    b: Double,
): Double = a - b

private fun prod(
    a: Double,
    b: Double,
): Double = a * b

private fun quot(
    a: Double,
    b: Double,
): Double = a / b

/** The fraction of the statement, written as pure nested calls. */
public fun ex_1_02(): Double =
    quot(
        sum(sum(5.0, 4.0), diff(2.0, diff(3.0, sum(6.0, quot(4.0, 5.0))))),
        prod(prod(3.0, diff(6.0, 2.0)), diff(2.0, 7.0)),
    )
