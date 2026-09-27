// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.13

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.vlist

public class E3_13Test :
    FunSpec({
        test("z closes back onto its first pair after exactly three cdrs") {
            val z = makeCycle(vlist(VSym("a"), VSym("b"), VSym("c")) as VPair)
            val second = z.cdr as VPair
            val third = second.cdr as VPair

            (third.cdr === z) shouldBe true
            (second === z) shouldBe false
            (third === z) shouldBe false
        }

        test("the cycle never reaches VNil, so lastPair would spin forever") {
            val z = makeCycle(vlist(VSym("a"), VSym("b"), VSym("c")) as VPair)

            var cursor: Value = z
            repeat(9) {
                cursor = (cursor as VPair).cdr
                (cursor === VNil) shouldBe false
            }
            (cursor === z) shouldBe true
        }
    })
