// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.23

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.vlist

public class E2_23Test :
    FunSpec({
        test("forEachValue visits (57 321 88) left to right") {
            ex_2_23() shouldBe listOf(57L, 321L, 88L)
        }
        test("forEachValue on the empty chain runs the action zero times") {
            val runs = mutableListOf<Long>()
            forEachValue(sicp.runtime.VNil) { v -> runs.add(numOf(v)) }
            runs.size shouldBe 0
        }
    })
