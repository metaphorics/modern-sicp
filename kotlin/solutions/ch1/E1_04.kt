// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.4

package sicp.ch1.exercises

/**
 * The operator is itself an expression: the `if` evaluates to a function
 * reference, `Long::plus` or `Long::minus`, and the call applies whichever
 * one the test produced.
 */
public fun ex_1_04(
    a: Long,
    b: Long,
): Long {
    val op: Long.(Long) -> Long = if (b > 0L) Long::plus else Long::minus
    return op(a, b)
}
