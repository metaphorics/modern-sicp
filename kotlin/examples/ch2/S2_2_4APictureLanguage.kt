// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.2.4

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** `wave2`: `wave` beside its own vertical flip, as in Figure 2.12. */
public val wave2: Painter = beside(wave, flipVert(wave))

/** `wave4`: `wave2` below itself. */
public val wave4: Painter = below(wave2, wave2)

private val probe: Painter =
    segmentsToPainter(listOf(makeSegment(makeVect(0.25, 0.25), makeVect(0.75, 0.75))))

private fun seg(
    x1: Double,
    y1: Double,
    x2: Double,
    y2: Double,
): Segment = makeSegment(makeVect(x1, y1), makeVect(x2, y2))

private fun waveCopy(map: (Vect) -> Vect): List<Segment> = waveSegments.map { makeSegment(map(it.start), map(it.end)) }

public class S2_2_4APictureLanguageTest :
    FunSpec({
        test("wave paints its 14 given segments unchanged into the unit square") {
            renderSegments(wave, unitSquare) shouldBe waveSegments
        }
        test("flippedPairs agrees with flippedPairsViaSquareOfFour, segment for segment") {
            renderSegments(flippedPairs(wave), unitSquare) shouldBe
                renderSegments(flippedPairsViaSquareOfFour(wave), unitSquare)
        }
        test("rightSplit(wave, n) paints 2^(n+1) - 1 copies") {
            renderSegments(rightSplit(wave, 0), unitSquare).size shouldBe 1 * 14
            renderSegments(rightSplit(wave, 1), unitSquare).size shouldBe 3 * 14
            renderSegments(rightSplit(wave, 2), unitSquare).size shouldBe 7 * 14
        }
        test("squareLimit agrees with squareLimitViaSquareOfFour, segment for segment") {
            renderSegments(squareLimit(wave, 1), unitSquare) shouldBe
                renderSegments(squareLimitViaSquareOfFour(wave, 1), unitSquare)
        }
        test("squareLimit(wave, n) paints 4 times cornerSplit(wave, n)'s segments") {
            val corner = renderSegments(cornerSplit(wave, 2), unitSquare).size
            renderSegments(squareLimit(wave, 2), unitSquare).size shouldBe 4 * corner
        }
        test("segmentsToPainter maps both endpoints into a translated, skewed frame") {
            val frame = makeFrame(makeVect(2.0, 3.0), makeVect(4.0, 1.0), makeVect(-1.0, 5.0))
            renderSegments(probe, frame) shouldBe listOf(seg(2.75, 4.5, 4.25, 7.5))
        }
        test("beside draws its two painters in the exact left and right halves") {
            renderSegments(beside(probe, probe), unitSquare) shouldBe
                listOf(
                    seg(0.125, 0.25, 0.375, 0.75),
                    seg(0.625, 0.25, 0.875, 0.75),
                )
        }
        test("below draws its two painters in the exact bottom and top halves") {
            renderSegments(below(probe, probe), unitSquare) shouldBe
                listOf(
                    seg(0.25, 0.125, 0.75, 0.375),
                    seg(0.25, 0.625, 0.75, 0.875),
                )
        }
        test("rightSplit(p, 1) places painter and two smaller copies at exact coordinates") {
            renderSegments(rightSplit(probe, 1), unitSquare) shouldBe
                listOf(
                    seg(0.125, 0.25, 0.375, 0.75),
                    seg(0.625, 0.125, 0.875, 0.375),
                    seg(0.625, 0.625, 0.875, 0.875),
                )
        }
        test("cornerSplit(p, 1) places its six copies at exact coordinates") {
            renderSegments(cornerSplit(probe, 1), unitSquare) shouldBe
                listOf(
                    seg(0.125, 0.125, 0.375, 0.375),
                    seg(0.0625, 0.625, 0.1875, 0.875),
                    seg(0.3125, 0.625, 0.4375, 0.875),
                    seg(0.625, 0.0625, 0.875, 0.1875),
                    seg(0.625, 0.3125, 0.875, 0.4375),
                    seg(0.625, 0.625, 0.875, 0.875),
                )
        }
        test("squareLimit(p, 0) faces its four copies outward at exact coordinates") {
            renderSegments(squareLimit(probe, 0), unitSquare) shouldBe
                listOf(
                    seg(0.375, 0.375, 0.125, 0.125),
                    seg(0.625, 0.375, 0.875, 0.125),
                    seg(0.375, 0.625, 0.125, 0.875),
                    seg(0.625, 0.625, 0.875, 0.875),
                )
        }
        test("wave4's four quarters are wave transformed exactly, segment for segment") {
            val bottomLeft: (Vect) -> Vect = { v -> makeVect(0.5 * v.x, 0.5 * v.y) }
            val bottomRight: (Vect) -> Vect = { v -> makeVect(0.5 + 0.5 * v.x, 0.5 * (1.0 - v.y)) }
            val topLeft: (Vect) -> Vect = { v -> makeVect(0.5 * v.x, 0.5 + 0.5 * v.y) }
            val topRight: (Vect) -> Vect = { v -> makeVect(0.5 + 0.5 * v.x, 0.5 + 0.5 * (1.0 - v.y)) }
            renderSegments(wave4, unitSquare) shouldBe
                waveCopy(bottomLeft) + waveCopy(bottomRight) + waveCopy(topLeft) + waveCopy(topRight)
        }
    })
