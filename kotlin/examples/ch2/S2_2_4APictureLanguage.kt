// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.2.4

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import java.nio.file.Files
import java.nio.file.Path

/** `wave2`: `wave` beside its own vertical flip, as in Figure 2.12. */
public val wave2: Painter = beside(wave, flipVert(wave))

/** `wave4`: `wave2` below itself. */
public val wave4: Painter = below(wave2, wave2)

/**
 * Walks up from the JVM working directory to the repository root (the
 * ancestor holding both `kotlin/` and `docs/exercise-map.md`) and returns
 * `book/figures/generated` under it, creating the directory if needed.
 */
private fun figuresDir(): Path {
    var dir = Path.of(System.getProperty("user.dir")).toAbsolutePath()
    while (!Files.isRegularFile(dir.resolve("docs/exercise-map.md"))) {
        dir = dir.parent ?: error("repository root not found above ${System.getProperty("user.dir")}")
    }
    val out = dir.resolve("book/figures/generated")
    Files.createDirectories(out)
    return out
}

private fun writeFigure(
    name: String,
    svg: String,
) {
    Files.writeString(figuresDir().resolve(name), svg)
}

public class S2_2_4APictureLanguageTest :
    FunSpec({
        test("wave paints its 14 given segments into a frame") {
            renderSegments(wave, unitSquare).size shouldBe 14
        }
        test("wave2 is wave beside its own flip: twice the segments") {
            renderSegments(wave2, unitSquare).size shouldBe 2 * 14
        }
        test("wave4 is wave2 below itself: four times the segments") {
            renderSegments(wave4, unitSquare).size shouldBe 4 * 14
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
        test("cornerSplit(wave, 1) paints 6 copies: painter + topLeft(2) + bottomRight(2) + corner(1)") {
            renderSegments(cornerSplit(wave, 1), unitSquare).size shouldBe 6 * 14
        }
        test("squareLimit agrees with squareLimitViaSquareOfFour, segment for segment") {
            renderSegments(squareLimit(wave, 1), unitSquare) shouldBe
                renderSegments(squareLimitViaSquareOfFour(wave, 1), unitSquare)
        }
        test("squareLimit(wave, n) paints 4 times cornerSplit(wave, n)'s segments") {
            val corner = renderSegments(cornerSplit(wave, 2), unitSquare).size
            renderSegments(squareLimit(wave, 2), unitSquare).size shouldBe 4 * corner
        }
        test("renderSvg wraps a painter's lines in a complete, well-formed SVG document") {
            val svg = renderSvg(wave4, width = 200, height = 200)
            svg.startsWith("<svg") shouldBe true
            svg.endsWith("</svg>") shouldBe true
            svg.contains("<line") shouldBe true
        }
        test("the section's figures render and are checked in under book/figures/generated") {
            writeFigure("kotlin-2-10-wave.svg", renderSvg(wave))
            writeFigure("kotlin-2-12-wave4.svg", renderSvg(wave4))
            writeFigure("kotlin-2-13-right-split.svg", renderSvg(rightSplit(wave, 4)))
            writeFigure("kotlin-2-14-corner-split.svg", renderSvg(cornerSplit(wave, 4)))
            writeFigure("kotlin-2-9-square-limit.svg", renderSvg(squareLimit(wave, 4)))
            for (name in listOf(
                "kotlin-2-10-wave.svg",
                "kotlin-2-12-wave4.svg",
                "kotlin-2-13-right-split.svg",
                "kotlin-2-14-corner-split.svg",
                "kotlin-2-9-square-limit.svg",
            )) {
                val path = figuresDir().resolve(name)
                Files.exists(path) shouldBe true
                Files.readString(path).startsWith("<svg") shouldBe true
            }
        }
    })
