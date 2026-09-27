// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.51

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_51Test :
    FunSpec({
        test("belowViaTransform matches the shared file's given below") {
            renderSegments(belowViaTransform(wave, xPainter), unitSquare) shouldBe
                renderSegments(below(wave, xPainter), unitSquare)
        }
        test("belowViaRotation matches belowViaTransform, segment for segment, for wave below xPainter") {
            renderSegments(belowViaRotation(wave, xPainter), unitSquare) shouldBe
                renderSegments(belowViaTransform(wave, xPainter), unitSquare)
        }
        test("belowViaRotation matches belowViaTransform for a different pair, diamondPainter below outlinePainter") {
            renderSegments(belowViaRotation(diamondPainter, outlinePainter), unitSquare) shouldBe
                renderSegments(belowViaTransform(diamondPainter, outlinePainter), unitSquare)
        }
        test("the first painter draws in the bottom half and the second in the top half") {
            val combined = renderSegments(belowViaTransform(xPainter, outlinePainter), unitSquare)
            val bottomHalf = combined.take(2) // xPainter's 2 segments, drawn first
            val topHalf = combined.drop(2) // outlinePainter's 4 segments, drawn second
            bottomHalf.forEach { segment ->
                (startSegment(segment).y <= 0.5 && endSegment(segment).y <= 0.5) shouldBe true
            }
            topHalf.forEach { segment ->
                (startSegment(segment).y >= 0.5 && endSegment(segment).y >= 0.5) shouldBe true
            }
        }
        test("ex_2_51 reports agreement") {
            ex_2_51() shouldBe true
        }
    })
