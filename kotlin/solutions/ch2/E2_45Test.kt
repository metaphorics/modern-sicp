// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.45

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_45Test :
    FunSpec({
        test("rightSplitViaSplit agrees with the given rightSplit, segment for segment") {
            renderSegments(rightSplitViaSplit(wave, 2), unitSquare) shouldBe
                renderSegments(rightSplit(wave, 2), unitSquare)
        }
        test("upSplitViaSplit agrees with exercise 2.44's upSplit, segment for segment") {
            renderSegments(upSplitViaSplit(wave, 2), unitSquare) shouldBe
                renderSegments(upSplit(wave, 2), unitSquare)
        }
        test("split(op, op)(painter, 0) is the painter itself") {
            renderSegments(rightSplitViaSplit(wave, 0), unitSquare) shouldBe renderSegments(wave, unitSquare)
        }
        test("ex_2_45 matches rightSplit(wave, 1)") {
            ex_2_45() shouldBe 3 * waveSegments.size
        }
    })
