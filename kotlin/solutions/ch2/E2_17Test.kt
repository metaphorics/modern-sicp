// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.17

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Whole
import sicp.runtime.datumList

public class E2_17Test :
    FunSpec({
        test("lastPair returns the final one-element suffix") {
            val input = datumList(Whole(23L), Whole(72L), Whole(149L), Whole(34L))
            val result = lastPair(input) as PairCell
            result.first shouldBe Whole(34L)
            result.second shouldBe Empty
        }
        test("lastPair preserves the original cell for a one-element chain") {
            val input = datumList(Whole(9L))
            (lastPair(input) === input) shouldBe true
        }
        test("ex_2_17 uses the canonical native datum rendering") {
            ex_2_17() shouldBe "PairCell(first=Whole(value=34), second=Empty)"
        }
    })
