// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.47

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_47Test :
    FunSpec({
        test("makeFramePair's selectors recover what the constructor was given") {
            val frame = makeFramePair(makeVect(1.0, 1.0), makeVect(2.0, 0.0), makeVect(0.0, 3.0))
            originFramePair(frame) shouldBe Vect(1.0, 1.0)
            edge1FramePair(frame) shouldBe Vect(2.0, 0.0)
            edge2FramePair(frame) shouldBe Vect(0.0, 3.0)
        }
        test("frameCoordMapPair agrees with frameCoordMap on the unit square's corners") {
            val origin = makeVect(1.0, 1.0)
            val edge1 = makeVect(2.0, 0.0)
            val edge2 = makeVect(0.0, 3.0)
            val listFrame = Frame(origin, edge1, edge2)
            val pairFrame = makeFramePair(origin, edge1, edge2)
            for (corner in listOf(makeVect(0.0, 0.0), makeVect(1.0, 0.0), makeVect(0.0, 1.0), makeVect(1.0, 1.0))) {
                frameCoordMap(listFrame)(corner) shouldBe frameCoordMapPair(pairFrame)(corner)
            }
        }
        test("ex_2_47 reports agreement") {
            ex_2_47() shouldBe true
        }
    })
