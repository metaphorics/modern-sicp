// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.21

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.renderDatum
import sicp.runtime.structurallyEqual

public class E2_21Test :
    FunSpec({
        test("both squareList definitions preserve the input order") {
            val input = datumList(Whole(1L), Whole(2L), Whole(3L), Whole(4L))
            val expected = datumList(Whole(1L), Whole(4L), Whole(9L), Whole(16L))
            structurallyEqual(squareList(input), expected) shouldBe true
            structurallyEqual(squareListViaMap(input), expected) shouldBe true
        }
        test("the two definitions agree on negative values") {
            val input = datumList(Whole(-10L), Whole(2L), Whole(-11L), Whole(17L))
            structurallyEqual(squareList(input), squareListViaMap(input)) shouldBe true
        }
        test("ex_2_21 returns the canonical native rendering") {
            ex_2_21() shouldBe renderDatum(datumList(Whole(1L), Whole(4L), Whole(9L), Whole(16L)))
        }
    })
