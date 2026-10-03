// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.22

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Empty
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.renderDatum
import sicp.runtime.structurallyEqual

public class E2_22Test :
    FunSpec({
        test("reverse accumulation produces the expected reversed chain") {
            val input = datumList(Whole(1L), Whole(2L), Whole(3L), Whole(4L))
            structurallyEqual(squareListIter(input), datumList(Whole(16L), Whole(9L), Whole(4L), Whole(1L))) shouldBe true
        }
        test("swapped pair arguments produce an improper nested structure") {
            val input = datumList(Whole(1L), Whole(2L), Whole(3L), Whole(4L))
            val expected = pair(pair(pair(pair(Empty, Whole(1L)), Whole(4L)), Whole(9L)), Whole(16L))
            structurallyEqual(squareListIterSwapped(input), expected) shouldBe true
        }
        test("ex_2_22 preserves Louis's output through native rendering") {
            ex_2_22() shouldBe renderDatum(datumList(Whole(16L), Whole(9L), Whole(4L), Whole(1L)))
        }
    })
