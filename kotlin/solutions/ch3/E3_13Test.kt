// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.13

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList

public class E3_13Test :
    FunSpec({
        test("z closes back onto its first pair after exactly three links") {
            val z = makeCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c")) as PairCell)
            val second = z.second as PairCell
            val third = second.second as PairCell

            (third.second === z) shouldBe true
            (second === z) shouldBe false
            (third === z) shouldBe false
        }

        test("the cycle does not reach the empty-list terminator") {
            val z = makeCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c")) as PairCell)

            var cursor: Datum = z
            repeat(9) {
                cursor = (cursor as PairCell).second
                (cursor === Empty) shouldBe false
            }
            (cursor === z) shouldBe true
        }
    })
