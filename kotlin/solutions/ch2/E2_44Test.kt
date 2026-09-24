// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.44

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_44Test :
    FunSpec({
        test("upSplit(wave, 0) is wave itself: 14 segments") {
            renderSegments(upSplit(wave, 0), unitSquare) shouldBe renderSegments(wave, unitSquare)
        }
        test("upSplit(wave, n) paints 2^(n+1) - 1 copies: 1, 3, 7 for n = 0, 1, 2") {
            renderSegments(upSplit(wave, 0), unitSquare).size shouldBe 1 * waveSegments.size
            renderSegments(upSplit(wave, 1), unitSquare).size shouldBe 3 * waveSegments.size
            renderSegments(upSplit(wave, 2), unitSquare).size shouldBe 7 * waveSegments.size
        }
        test("cornerSplit(wave, 1), which calls the given up-split internally, still paints 6 copies") {
            // painter(1) + topLeft=beside(up,up)(2) + bottomRight=below(right,right)(2) + corner(1) = 6
            renderSegments(cornerSplit(wave, 1), unitSquare).size shouldBe 6 * waveSegments.size
        }
        test("ex_2_44 matches upSplit(wave, 1)") {
            ex_2_44() shouldBe 3 * waveSegments.size
        }
    })
