// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.59

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

/** A finite coefficient stream. */
private fun seriesOf(vararg coefficients: Rat): LStream<Rat> {
    fun build(i: Int): LStream<Rat> = if (i == coefficients.size) LStream.Empty else consStream(coefficients[i]) { build(i + 1) }
    return build(0)
}

public class E3_59Test :
    FunSpec({
        test("integrateSeries divides each coefficient by its index plus one") {
            integrateSeries(seriesOf(Rat.ONE, Rat.ONE, Rat.ONE)).take(3) shouldBe
                listOf(Rat.ONE, Rat.of(1L, 2L), Rat.of(1L, 3L))
        }

        test("the exponential series is its own integral past the constant 1") {
            expSeries.take(8) shouldBe
                listOf(
                    Rat.ONE,
                    Rat.ONE,
                    Rat.of(1L, 2L),
                    Rat.of(1L, 6L),
                    Rat.of(1L, 24L),
                    Rat.of(1L, 120L),
                    Rat.of(1L, 720L),
                    Rat.of(1L, 5040L),
                )
        }

        test("the sine series is the integral of the cosine series past the constant 0") {
            sinSeries.take(6) shouldBe
                listOf(Rat.ZERO, Rat.ONE, Rat.ZERO, Rat.of(-1L, 6L), Rat.ZERO, Rat.of(1L, 120L))
        }

        test("the cosine series is the integral of the negated sine series past the constant 1") {
            cosSeries.take(5) shouldBe listOf(Rat.ONE, Rat.ZERO, Rat.of(-1L, 2L), Rat.ZERO, Rat.of(1L, 24L))
        }
    })
