// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.59a

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.consStream
import sicp.runtime.take

public class E3_59aTest :
    FunSpec({
        test("the derivative of exp is exp itself").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            differentiateSeries(expSeries).take(8) shouldBe expSeries.take(8)
        }

        test("the derivative of sine is cosine").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            differentiateSeries(sinSeries).take(6) shouldBe cosSeries.take(6)
        }

        test("the derivative of cosine is the negated sine").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            differentiateSeries(cosSeries).take(6) shouldBe sinSeries.take(6).map { -it }
        }

        test("integrating then differentiating restores any series past its constant").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            listOf(expSeries, sinSeries, cosSeries).forEach { series ->
                val integral = consStream(Rat.ZERO) { integrateSeries(series) }
                differentiateSeries(integral).take(6) shouldBe series.take(6)
            }
        }
    })
