// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.46a

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.double
import io.kotest.property.arbitrary.filter
import io.kotest.property.checkAll
import kotlin.math.abs

private val coordinate: Arb<Double> = Arb.double(-1.0e6, 1.0e6).filter { it.isFinite() }

private val smallCoordinate: Arb<Double> = Arb.double(-1.0e3, 1.0e3).filter { it.isFinite() }

private val scalar: Arb<Double> = Arb.double(-100.0, 100.0).filter { it.isFinite() }

private fun near(
    a: Vect,
    b: Vect,
): Boolean = abs(a.x - b.x) < 1e-9 && abs(a.y - b.y) < 1e-9

public class E2_46aTest :
    FunSpec({
        test("+ agrees with addVect") {
            makeVect(1.0, 2.0) + makeVect(3.0, 4.0) shouldBe addVect(makeVect(1.0, 2.0), makeVect(3.0, 4.0))
        }
        test("- agrees with subVect") {
            makeVect(5.0, 5.0) - makeVect(2.0, 1.0) shouldBe subVect(makeVect(5.0, 5.0), makeVect(2.0, 1.0))
        }
        test("* agrees with scaleVect") {
            makeVect(2.0, -3.0) * 4.0 shouldBe scaleVect(makeVect(2.0, -3.0), 4.0)
        }
        test("+ agrees with addVect for every generated pair") {
            checkAll(coordinate, coordinate, coordinate, coordinate) {
                x1,
                y1,
                x2,
                y2,
                ->
                val a = makeVect(x1, y1)
                val b = makeVect(x2, y2)
                (a + b) shouldBe addVect(a, b)
            }
        }
        test("vector addition is commutative: a + b == b + a") {
            checkAll(coordinate, coordinate, coordinate, coordinate) {
                x1,
                y1,
                x2,
                y2,
                ->
                val a = makeVect(x1, y1)
                val b = makeVect(x2, y2)
                (a + b) shouldBe (b + a)
            }
        }
        test("subtraction undoes addition: (a + b) - b == a") {
            checkAll(coordinate, coordinate, coordinate, coordinate) {
                x1,
                y1,
                x2,
                y2,
                ->
                val a = makeVect(x1, y1)
                val b = makeVect(x2, y2)
                near((a + b) - b, a) shouldBe true
            }
        }
        test("scale distributes over addition: (a + b) * s == a * s + b * s") {
            checkAll(
                smallCoordinate,
                smallCoordinate,
                smallCoordinate,
                smallCoordinate,
                scalar,
            ) { x1, y1, x2, y2, s ->
                val a = makeVect(x1, y1)
                val b = makeVect(x2, y2)
                near((a + b) * s, a * s + b * s) shouldBe true
            }
        }
        test("ex_2_46a matches makeVect(1.0, 2.0) + makeVect(3.0, 4.0)") {
            ex_2_46a() shouldBe Vect(4.0, 6.0)
        }
    })
