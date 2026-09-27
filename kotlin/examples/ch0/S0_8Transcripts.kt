// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** The arithmetic session of section 0.8, computed for real. */
public val sessionArithmetic: List<Long> =
    listOf(
        486L,
        100L,
        5L + 3L + 4L,
        10L - 9L,
        2L * 4L + (4L - 6L),
    )

/** The `square` session of section 0.8. */
public fun sessionSquare(): List<Long> {
    fun square(x: Long): Long = x * x
    return listOf(
        square(21L),
        square(2L + 5L),
        square(square(3L)),
    )
}

public class S0_8TranscriptsTest :
    FunSpec({
        test("the arithmetic session evaluates in order") {
            sessionArithmetic shouldBe listOf(486L, 100L, 12L, 1L, 6L)
        }
        test("the square session evaluates in order") {
            sessionSquare() shouldBe listOf(441L, 49L, 81L)
        }
    })
