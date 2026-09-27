// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.48

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_48Test :
    FunSpec({
        test("the selectors recover the endpoints the constructor was given") {
            val s = makeSegment(makeVect(1.0, 2.0), makeVect(4.0, 6.0))
            startSegment(s) shouldBe Vect(1.0, 2.0)
            endSegment(s) shouldBe Vect(4.0, 6.0)
        }
        test("a zero-length segment has length 0") {
            segmentLength(makeSegment(makeVect(2.0, 2.0), makeVect(2.0, 2.0))) shouldBe 0.0
        }
        test("ex_2_48 is the length of the (0,0)-(3,4) segment, 5.0") {
            ex_2_48() shouldBe 5.0
        }
    })
