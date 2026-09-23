// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E0_05Test :
    FunSpec({
        test("both shapes produce the same five squares") {
            squaresSeq { }.take(5).toList() shouldBe listOf(1L, 4L, 9L, 16L, 25L)
            squaresStream { }.take(5) shouldBe listOf(1L, 4L, 9L, 16L, 25L)
        }
        test("a second walk recomputes the Sequence but not the LStream") {
            ex_0_05() shouldBe Pair(10, 6)
        }
        test("the memoized stream returns the same nodes on a second walk") {
            val stream = squaresStream { }
            stream.take(5) shouldBe stream.take(5)
        }
    })
