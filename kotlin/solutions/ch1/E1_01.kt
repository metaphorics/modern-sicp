// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.1

package sicp.ch1.exercises

/** The five arithmetic expressions of the sequence, evaluated in order. */
private fun arithmetic(): List<Long> =
    listOf(
        10L,
        5L + 3L + 4L,
        9L - 1L,
        6L / 2L,
        2L * 4L + (4L - 6L),
    )

/** The names session: `a` bound to 3, `b` to 4, then five expressions. */
private fun names(): List<Long> {
    val a = 3L
    val b = a + 1L
    return listOf(
        a + b + (a * b),
        if (b > a && b < a * b) b else a,
        when {
            a == 4L -> 6L
            b == 4L -> 6L + 7L + a
            else -> 25L
        },
        2L + (if (b > a) b else a),
        (
            when {
                a > b -> a
                a < b -> b
                else -> -1L
            }
        ) * (a + 1L),
    )
}

/** The ten numeric results of the sequence, in presentation order. */
public fun ex_1_01(): List<Long> = arithmetic() + names()
