// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.50

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

private fun mirrored(v: Vect): Vect = makeVect(1.0 - v.x, v.y)

private fun rotated180(v: Vect): Vect = makeVect(1.0 - v.x, 1.0 - v.y)

private fun rotated270(v: Vect): Vect = makeVect(v.y, 1.0 - v.x)

private fun expectedSegments(map: (Vect) -> Vect): List<Segment> =
    waveSegments.map { makeSegment(map(startSegment(it)), map(endSegment(it))) }

public class E2_50Test :
    FunSpec({
        test("flipHoriz mirrors every coordinate: x becomes 1 - x, in the same draw order") {
            renderSegments(flipHoriz(wave), unitSquare) shouldBe expectedSegments(::mirrored)
        }
        test("rotate180 sends (x, y) to (1 - x, 1 - y), in the same draw order") {
            renderSegments(rotate180(wave), unitSquare) shouldBe expectedSegments(::rotated180)
        }
        test("rotate270 sends (x, y) to (y, 1 - x), in the same draw order") {
            renderSegments(rotate270(wave), unitSquare) shouldBe expectedSegments(::rotated270)
        }
        test("rotate180 is flipVert composed with flipHoriz, the prose's footnote about compose") {
            renderSegments(rotate180(wave), unitSquare) shouldBe
                renderSegments(flipVert(flipHoriz(wave)), unitSquare)
        }
        test("ex_2_50 matches rotate180(wave)'s segment count, 14") {
            ex_2_50() shouldBe 14
        }
    })
