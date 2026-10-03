// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.23

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Empty
import sicp.runtime.Whole
import sicp.runtime.datumList

public class E2_23Test :
    FunSpec({
        test("forEachValue visits the sample values from left to right") {
            ex_2_23() shouldBe listOf(57L, 321L, 88L)
        }
        test("forEachValue on Empty runs the action zero times") {
            val runs = mutableListOf<Long>()
            forEachValue(Empty) { value -> runs.add(wholeNumber(value)) }
            runs.size shouldBe 0
        }
        test("forEachValue receives each typed numeric datum unchanged") {
            val seen = mutableListOf<Whole>()
            forEachValue(datumList(Whole(4L), Whole(1L))) { value -> seen.add(value as Whole) }
            seen shouldBe listOf(Whole(4L), Whole(1L))
        }
    })
