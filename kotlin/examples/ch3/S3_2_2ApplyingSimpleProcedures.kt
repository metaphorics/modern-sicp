// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.2.2, applying simple procedures

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * The book's `sum-of-squares`. Each call opens a frame binding `x` and
 * `y`, and the two calls of `square` inside it each open a frame of
 * their own, all extending the global environment where the three
 * functions were declared.
 */
public fun sumOfSquares(
    x: Long,
    y: Long,
): Long = square(x) + square(y)

/** The book's `f`. */
public fun f(a: Long): Long = sumOfSquares(a + 1, a * 2)

public class S3_2_2ApplyingSimpleProceduresTest :
    FunSpec({
        test("the book's walkthrough: f(5) is 136") {
            f(5L) shouldBe 136L
        }

        test("the two calls of square inside sumOfSquares(6, 10) give 36 and 100") {
            sumOfSquares(6L, 10L) shouldBe 136L
            square(6L) shouldBe 36L
            square(10L) shouldBe 100L
        }

        test("f threads a fresh a into sumOfSquares(a + 1, a * 2) on every call") {
            f(1L) shouldBe 8L
            f(5L) shouldBe 136L
        }
    })
