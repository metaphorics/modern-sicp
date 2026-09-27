// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.6

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_06Test :
    FunSpec({
        test("reset replays the same two-word sequence") {
            val r = makeResettableRand(7UL)
            val a1 = r.generate()
            val a2 = r.generate()
            r.reset(7UL)
            r.generate() shouldBe a1
            r.generate() shouldBe a2
        }

        test("generate advances the state on every call") {
            val r = makeResettableRand(7UL)
            val a1 = r.generate()
            val a2 = r.generate()
            (a1 == a2) shouldBe false
        }

        test("reset can jump to an arbitrary word, not just the original seed") {
            val r = makeResettableRand(7UL)
            r.generate()
            r.reset(99UL)
            val fromReset = r.generate()
            val expected = makeResettableRand(99UL).generate()
            fromReset shouldBe expected
        }
    })
