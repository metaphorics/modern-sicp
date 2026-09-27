// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** A binding: a name for a value. A `val` cannot be reassigned. */
public val planck: Double = 6.626_070_15e-34

/** The book's division rule: `Long / Long` truncates toward zero. */
public fun halfOf(n: Long): Long = n / 2L

/** Real division needs a real operand. */
public fun realHalf(n: Long): Double = n / 2.0

/** `if` is an expression: it produces a value. */
public fun sign(n: Long): String =
    if (n > 0L) {
        "positive"
    } else if (n < 0L) {
        "negative"
    } else {
        "zero"
    }

/** `when` matches one value against candidates in order. */
public fun classify(n: Long): String =
    when (n) {
        0L -> "zero"
        in 1L..9L -> "small"
        else -> "large"
    }

/** A string template splices values into text. */
public fun describe(n: Long): String = "n is $n"

public class S0_2NamesTest :
    FunSpec({
        test("bindings hold literals of each kind") {
            planck shouldBe 6.626_070_15e-34
        }
        test("integer division truncates, real division does not") {
            halfOf(9) shouldBe 4L
            realHalf(9) shouldBe 4.5
        }
        test("if and when produce values") {
            sign(-3) shouldBe "negative"
            sign(0) shouldBe "zero"
            classify(7) shouldBe "small"
            classify(42) shouldBe "large"
        }
        test("templates render values") {
            describe(21) shouldBe "n is 21"
        }
    })
