// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.49

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

private fun endpoints(segments: List<Segment>): Set<Vect> = segments.flatMap { listOf(startSegment(it), endSegment(it)) }.toSet()

public class E2_49Test :
    FunSpec({
        test("outlinePainter has 4 segments touching all 4 corners of the unit square") {
            val drawn = renderSegments(outlinePainter, unitSquare)
            drawn.size shouldBe 4
            endpoints(drawn) shouldBe
                setOf(makeVect(0.0, 0.0), makeVect(1.0, 0.0), makeVect(1.0, 1.0), makeVect(0.0, 1.0))
        }
        test("xPainter has 2 diagonals, connecting opposite corners") {
            val drawn = renderSegments(xPainter, unitSquare)
            drawn.size shouldBe 2
            endpoints(drawn) shouldBe
                setOf(makeVect(0.0, 0.0), makeVect(1.0, 0.0), makeVect(1.0, 1.0), makeVect(0.0, 1.0))
        }
        test("diamondPainter has 4 segments touching the 4 side midpoints") {
            val drawn = renderSegments(diamondPainter, unitSquare)
            drawn.size shouldBe 4
            endpoints(drawn) shouldBe
                setOf(makeVect(0.5, 0.0), makeVect(1.0, 0.5), makeVect(0.5, 1.0), makeVect(0.0, 0.5))
        }
        test("the given wave painter has 14 segments, part (d) of the statement") {
            renderSegments(wave, unitSquare).size shouldBe 14
        }
        test("ex_2_49 reports [4, 2, 4]") {
            ex_2_49() shouldBe listOf(4, 2, 4)
        }
    })
