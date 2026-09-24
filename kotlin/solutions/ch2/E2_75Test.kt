// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.75

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlin.math.atan2

public class E2_75Test :
    FunSpec({
        test("ex_2_75 answers the magnitude and angle of the polar number it built") {
            ex_2_75() shouldBe (5.0 to atan2(4.0, 3.0))
        }

        test("makeFromMagAngMessagePassing answers real-part and imag-part too, analogous to make-from-real-imag") {
            val z = makeFromMagAngMessagePassing(5.0, 0.0)
            applyGenericMessagePassing("real-part", z) shouldBe 5.0
            applyGenericMessagePassing("imag-part", z) shouldBe 0.0
        }

        test("an unanswered message returns the absent option, matching make-from-real-imag's own dispatch") {
            val z = makeFromMagAngMessagePassing(5.0, 0.0)
            applyGenericMessagePassing("rotate", z) shouldBe null
        }
    })
