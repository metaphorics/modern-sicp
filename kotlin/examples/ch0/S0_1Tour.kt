// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** The section 0.1 tour: `square`. */
public fun tourSquare(x: Long): Long = x * x

/**
 * The listing file of section 0.1 shows the whole anatomy in miniature:
 * the definitions the book prose displays, and one Kotest spec asserting
 * every `// =>` result the prose annotates. Nothing else ships with a
 * listing; there is no hidden test rig beyond the `FunSpec` you see.
 */
public class S0_1TourTest :
    FunSpec({
        test("the call returns the value the prose annotates") {
            tourSquare(21) shouldBe 441L
            tourSquare(2L + 5L) shouldBe 49L
            tourSquare(tourSquare(3)) shouldBe 81L
        }
    })
