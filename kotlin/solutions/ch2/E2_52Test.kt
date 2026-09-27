// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.52

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.shouldNotBe

public class E2_52Test :
    FunSpec({
        test("(a) waveWithSmile has the wave's 14 segments plus 2 smile segments") {
            renderSegments(waveWithSmile, unitSquare).size shouldBe (waveSegments.size + 2)
        }
        test("(b) cornerSplitModified(wave, 1) uses one copy each of up and right: 4 copies, versus the book's 6") {
            val modified = renderSegments(cornerSplitModified(wave, 1), unitSquare)
            val original = renderSegments(cornerSplit(wave, 1), unitSquare)
            modified.size shouldBe 4 * waveSegments.size
            // painter(1) + topLeft=beside(up,up)(2) + bottomRight=below(right,right)(2) + corner(1) = 6
            original.size shouldBe 6 * waveSegments.size
        }
        test("(c) squareLimitModified paints 4 corner copies of cornerSplitModified, each the same size") {
            val corner = renderSegments(cornerSplitModified(waveWithSmile, 1), unitSquare).size
            corner shouldBe 4 * waveSegmentsWithSmile.size
            renderSegments(squareLimitModified(waveWithSmile, 1), unitSquare).size shouldBe 4 * corner
        }
        test("(c) squareLimitModified differs from the book's squareLimit for the same painter and depth") {
            renderSegments(squareLimitModified(waveWithSmile, 1), unitSquare) shouldNotBe
                renderSegments(squareLimit(waveWithSmile, 1), unitSquare)
        }
        test("ex_2_52 matches squareLimitModified(waveWithSmile, 1)'s segment count, 256") {
            ex_2_52() shouldBe 256
        }
    })
