// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.53

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Symbol
import sicp.runtime.Truth
import sicp.runtime.datumList
import sicp.runtime.renderDatum

public class E2_53Test :
    FunSpec({
        val predictions = ex_2_53()

        test("prediction 1 renders a flat symbol sequence") {
            predictions[0] shouldBe renderDatum(datumList(Symbol("a"), Symbol("b"), Symbol("c")))
        }
        test("prediction 2 renders a singleton nested sequence") {
            predictions[1] shouldBe renderDatum(datumList(datumList(Symbol("george"))))
        }
        test("prediction 3 renders the tail containing the second inner sequence") {
            predictions[2] shouldBe renderDatum(datumList(datumList(Symbol("y1"), Symbol("y2"))))
        }
        test("prediction 4 renders the second inner sequence itself") {
            predictions[3] shouldBe renderDatum(datumList(Symbol("y1"), Symbol("y2")))
        }
        test("prediction 5 renders the false pair-shape observation") {
            predictions[4] shouldBe renderDatum(Truth(false))
        }
        test("prediction 6 does not find a symbol buried in a nested sequence") {
            predictions[5] shouldBe renderDatum(Truth(false))
        }
        test("prediction 7 renders the matching suffix of a flat sequence") {
            predictions[6] shouldBe renderDatum(datumList(Symbol("red"), Symbol("shoes"), Symbol("blue"), Symbol("socks")))
        }
    })
