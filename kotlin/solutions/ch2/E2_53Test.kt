// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.53 (replaced)

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VBool
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.vlist

private fun memq(
    item: Value,
    x: Value,
): Value =
    when {
        x is VPair && x.car == item -> x
        x is VPair -> memq(item, x.cdr)
        else -> VBool(false)
    }

public class E2_53Test :
    FunSpec({
        val predictions = ex_2_53()

        test("prediction 1: list of three symbols") {
            predictions[0] shouldBe vlist(VSym("a"), VSym("b"), VSym("c")).toString()
        }
        test("prediction 2: a singleton list holding a singleton list") {
            predictions[1] shouldBe vlist(vlist(VSym("george"))).toString()
        }
        test("predictions 3 and 4: cdr and cadr of the pairs list") {
            val pairs = vlist(vlist(VSym("x1"), VSym("x2")), vlist(VSym("y1"), VSym("y2")))
            val cdrOfPairs = (pairs as VPair).cdr
            predictions[2] shouldBe cdrOfPairs.toString()
            predictions[3] shouldBe (cdrOfPairs as VPair).car.toString()
        }
        test("prediction 5: the car of a short list is a symbol, not a pair") {
            val shortList = vlist(VSym("a"), VSym("short"), VSym("list"))
            val carOfShortList = (shortList as VPair).car
            predictions[4] shouldBe if (carOfShortList is VPair) "#t" else "#f"
        }
        test("prediction 6: memq fails when the match is buried inside a sublist") {
            val nested = vlist(vlist(VSym("red"), VSym("shoes")), vlist(VSym("blue"), VSym("socks")))
            predictions[5] shouldBe memq(VSym("red"), nested).toString()
        }
        test("prediction 7: memq finds a flat match") {
            val flat = vlist(VSym("red"), VSym("shoes"), VSym("blue"), VSym("socks"))
            predictions[6] shouldBe memq(VSym("red"), flat).toString()
        }
    })
