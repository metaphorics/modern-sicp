// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.62

package sicp.ch3.exercises

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_62Test :
    FunSpec({
        test("the tangent series is sine divided by cosine, exactly") {
            tangentSeries.take(10) shouldBe
                listOf(
                    Rat.ZERO,
                    Rat.ONE,
                    Rat.ZERO,
                    Rat.of(1L, 3L),
                    Rat.ZERO,
                    Rat.of(2L, 15L),
                    Rat.ZERO,
                    Rat.of(17L, 315L),
                    Rat.ZERO,
                    Rat.of(62L, 2835L),
                )
        }

        test("divSeries rejects a denominator with a zero constant term") {
            shouldThrow<IllegalArgumentException> { divSeries(expSeries, sinSeries) }
        }
    })
