// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.1.1

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * Kotlin's [Pair] stores two values directly. [Pair.first] and [Pair.second]
 * select them, and either field can itself contain a pair to build nested
 * compound data.
 */
public fun pairsPrimitiveDemo(): Long {
    val x = 1L to 2L
    check(x.first == 1L)
    check(x.second == 2L)
    val y = 3L to 4L
    val z = x to y
    check(z.first.first == 1L)
    check(z.second.first == 3L)
    return z.first.first + z.second.first
}

/**
 * A rational number stores an integer numerator and denominator. The initial
 * constructor preserves its inputs as given; reduction is a separate
 * representation choice introduced later in the section.
 */
public data class Rational(
    val numer: Long,
    val denom: Long,
)

public fun makeRat(
    n: Long,
    d: Long,
): Rational = Rational(n, d)

public fun addRat(
    x: Rational,
    y: Rational,
): Rational = makeRat(x.numer * y.denom + y.numer * x.denom, x.denom * y.denom)

public fun subRat(
    x: Rational,
    y: Rational,
): Rational = makeRat(x.numer * y.denom - y.numer * x.denom, x.denom * y.denom)

public fun mulRat(
    x: Rational,
    y: Rational,
): Rational = makeRat(x.numer * y.numer, x.denom * y.denom)

public fun divRat(
    x: Rational,
    y: Rational,
): Rational = makeRat(x.numer * y.denom, x.denom * y.numer)

public fun equalRat(
    x: Rational,
    y: Rational,
): Boolean = x.numer * y.denom == y.numer * x.denom

/** Print the numerator and denominator separated by a slash. */
public fun printRat(x: Rational) {
    println("${x.numer}/${x.denom}")
}

private tailrec fun gcd(
    a: Long,
    b: Long,
): Long = if (b == 0L) a else gcd(b, a % b)

/**
 * Reduce to lowest terms at construction using [gcd]. This distinct
 * constructor lets callers compare reduced and unreduced records without
 * changing the rational operations or their abstraction barrier.
 */
public fun makeRatReduced(
    n: Long,
    d: Long,
): Rational {
    val g = gcd(n, d)
    return Rational(n / g, d / g)
}

public class S2_1_1RationalNumbersTest :
    FunSpec({
        test("Kotlin pairs nest to represent compound data") {
            pairsPrimitiveDemo() shouldBe 4L
        }
        test("rational operations preserve the expected numerator-denominator values") {
            val oneHalf = makeRat(1L, 2L)
            val oneThird = makeRat(1L, 3L)
            "${oneHalf.numer}/${oneHalf.denom}" shouldBe "1/2"
            "${oneThird.numer}/${oneThird.denom}" shouldBe "1/3"
            val sum = addRat(oneHalf, oneThird)
            "${sum.numer}/${sum.denom}" shouldBe "5/6"
            val product = mulRat(oneHalf, oneThird)
            "${product.numer}/${product.denom}" shouldBe "1/6"
        }
        test("this makeRat does not reduce to lowest terms") {
            val oneThird = makeRat(1L, 3L)
            val sum = addRat(oneThird, oneThird)
            "${sum.numer}/${sum.denom}" shouldBe "6/9"
        }
        test("makeRatReduced fixes exactly that, reducing 6/9 to 2/3") {
            makeRatReduced(6L, 9L) shouldBe Rational(2L, 3L)
        }
        test("equalRat compares by cross-multiplication, not by field equality") {
            equalRat(makeRat(1L, 2L), makeRat(2L, 4L)) shouldBe true
            equalRat(makeRat(1L, 2L), makeRat(1L, 3L)) shouldBe false
        }
    })
