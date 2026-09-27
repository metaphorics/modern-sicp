// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.61

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

/** An endless supply of zero coefficients for fixture series. */
private val ratZeros: LStream<Rat> = consStream(Rat.ZERO) { ratZeros }

public class E3_61Test :
    FunSpec({
        test("the reciprocal of 1 - x is the all-ones series").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val oneMinusX = consStream(Rat.ONE) { consStream(Rat.of(-1L)) { ratZeros } }
            invertUnitSeries(oneMinusX).take(8) shouldBe List(8) { Rat.ONE }
        }

        test("a series times its reciprocal is the unit series").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            mulSeries(expSeries, invertUnitSeries(expSeries)).take(8) shouldBe
                listOf(Rat.ONE) + List(7) { Rat.ZERO }
        }
    })
