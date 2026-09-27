// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

/** Session 1: plain arithmetic, evaluated in order. */
private fun arithmeticSession(): List<Long> =
    listOf(
        486L,
        100L,
        5L + 3L + 4L,
        10L - 9L,
        2L * 4L + (4L - 6L),
    )

/** Session 2: names and conditionals, with `a` bound to 3 and `b` to 4. */
private fun namingSession(): List<Long> {
    val a = 3L
    val b = a + 1L
    return listOf(
        a + b + a * b,
        if (b > a && b < a * b) b else a,
        when {
            a == 4L -> 6L
            b == 4L -> 6L + 7L + a
            else -> 25L + a
        },
        2L + if (b > a) b else a,
        (
            when {
                a > b -> a
                a < b -> b
                else -> -1L
            }
        ) * (a + 1L),
    )
}

/** Session 3: `square` at work, including on its own result. */
private fun squareSession(): List<Long> {
    fun square(x: Long): Long = x * x
    return listOf(
        square(21L),
        square(2L + 5L),
        square(square(3L)),
    )
}

/** The three sessions in order; the Boolean `(= a b)` line is `false`. */
public fun ex_0_01(): List<Long> = arithmeticSession() + namingSession() + squareSession()
