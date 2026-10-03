// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.18

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.renderDatum
import sicp.runtime.structurallyEqual

public class E2_18Test :
    FunSpec({
        test("reverseList preserves every element in reverse order") {
            val input = datumList(Whole(1L), Whole(4L), Whole(9L), Whole(16L), Whole(25L))
            val expected = datumList(Whole(25L), Whole(16L), Whole(9L), Whole(4L), Whole(1L))
            structurallyEqual(reverseList(input), expected) shouldBe true
        }
        test("reversing twice restores the original structure") {
            val input = datumList(Whole(1L), Whole(4L), Whole(9L), Whole(16L), Whole(25L))
            structurallyEqual(reverseList(reverseList(input)), input) shouldBe true
        }
        test("ex_2_18 returns the canonical native rendering") {
            ex_2_18() shouldBe renderDatum(datumList(Whole(25L), Whole(16L), Whole(9L), Whole(4L), Whole(1L)))
        }
    })
