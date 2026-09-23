// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.1.4

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** To square something, multiply it by itself. */
public fun square(x: Long): Long = x * x

/** Built from [square] exactly as the text builds sum-of-squares. */
public fun sumOfSquares(
    x: Long,
    y: Long,
): Long = square(x) + square(y)

/** Built from [sumOfSquares] exactly as the text builds f. */
public fun f(a: Long): Long = sumOfSquares(a + 1L, a * 2L)

public class S1_1_4ProceduresTest :
    FunSpec({
        test("square applies to expressions and to its own results") {
            square(21L) shouldBe 441L
            square(2L + 5L) shouldBe 49L
            square(square(3L)) shouldBe 81L
        }
        test("each definition composes the ones before it") {
            sumOfSquares(3L, 4L) shouldBe 25L
            f(5L) shouldBe 136L
        }
    })
