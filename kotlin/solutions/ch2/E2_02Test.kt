// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.2

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_02Test :
    FunSpec({
        test("midpointSegment averages the endpoints' coordinates") {
            val s = makeSegment(makePoint(2.0, 3.0), makePoint(8.0, 11.0))
            midpointSegment(s) shouldBe Point(5.0, 7.0)
        }
        test("startSegment and endSegment recover the two endpoints given to makeSegment") {
            val a = makePoint(0.0, 0.0)
            val b = makePoint(4.0, 4.0)
            val s = makeSegment(a, b)
            startSegment(s) shouldBe a
            endSegment(s) shouldBe b
        }
        test("ex_2_02 matches the midpoint of (2, 3) to (8, 11)") {
            ex_2_02() shouldBe Point(5.0, 7.0)
        }
    })
