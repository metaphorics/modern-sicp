// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.60

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

/** A finite coefficient stream for the multiplication-table check. */
private fun seriesOf(vararg coefficients: Rat): LStream<Rat> {
    fun build(i: Int): LStream<Rat> = if (i == coefficients.size) LStream.Empty else consStream(coefficients[i]) { build(i + 1) }
    return build(0)
}

public class E3_60Test :
    FunSpec({
        test("the product of two finite series follows the convolution table").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            mulSeries(seriesOf(Rat.ONE, Rat.ONE, Rat.ONE), seriesOf(Rat.ONE, Rat.ONE, Rat.ONE)).take(3) shouldBe
                listOf(Rat.ONE, Rat.of(2L), Rat.of(3L))
            mulSeries(seriesOf(Rat.ONE, Rat.ONE, Rat.ONE), seriesOf(Rat.ONE, Rat.ONE)).take(2) shouldBe
                listOf(Rat.ONE, Rat.of(2L))
        }

        test("sin^2 + cos^2 is the unit series, exactly").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val sum =
                zipStream(
                    mulSeries(sinSeries, sinSeries),
                    mulSeries(cosSeries, cosSeries),
                ) { x, y -> x + y }
            sum.take(10) shouldBe listOf(Rat.ONE) + List(9) { Rat.ZERO }
        }
    })
