// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.46

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_46Test :
    FunSpec({
        test("the selectors recover what the constructor was given") {
            val v = makeVect(3.0, -2.5)
            xcorVect(v) shouldBe 3.0
            ycorVect(v) shouldBe -2.5
        }
        test("addVect adds coordinatewise") {
            addVect(makeVect(1.0, 2.0), makeVect(3.0, 4.5)) shouldBe Vect(4.0, 6.5)
        }
        test("subVect subtracts coordinatewise") {
            subVect(makeVect(5.0, 5.0), makeVect(2.0, 1.0)) shouldBe Vect(3.0, 4.0)
        }
        test("scaleVect multiplies both coordinates") {
            scaleVect(makeVect(2.0, -3.0), 4.0) shouldBe Vect(8.0, -12.0)
        }
        test("frameCoordMap maps the unit square's corners to the frame's corners, as the prose's example does") {
            val frame = Frame(makeVect(1.0, 1.0), makeVect(2.0, 0.0), makeVect(0.0, 2.0))
            val map = frameCoordMap(frame)
            map(makeVect(0.0, 0.0)) shouldBe originFrame(frame)
            map(makeVect(1.0, 0.0)) shouldBe addVect(originFrame(frame), edge1Frame(frame))
            map(makeVect(0.0, 1.0)) shouldBe addVect(originFrame(frame), edge2Frame(frame))
            map(makeVect(1.0, 1.0)) shouldBe addVect(addVect(originFrame(frame), edge1Frame(frame)), edge2Frame(frame))
        }
        test("ex_2_46 matches addVect(makeVect(1.0, 2.0), makeVect(3.0, 4.5))") {
            ex_2_46() shouldBe Vect(4.0, 6.5)
        }
    })
