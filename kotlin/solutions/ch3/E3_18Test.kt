// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.18

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.cons
import sicp.runtime.vlist

public class E3_18Test :
    FunSpec({
        test("the cycle made in 3.13 is detected") {
            val z = makeCycle(vlist(VSym("a"), VSym("b"), VSym("c")) as VPair)

            containsCycle(z) shouldBe true
        }

        test("plain chains are reported acyclic, however long") {
            containsCycle(vlist(VSym("a"), VSym("b"), VSym("c"))) shouldBe false
            containsCycle(
                vlist(
                    VSym("a"),
                    VSym("b"),
                    VSym("c"),
                    VSym("d"),
                    VSym("e"),
                    VSym("f"),
                    VSym("g"),
                    VSym("h"),
                ),
            ) shouldBe false
        }

        test("sharing through the car only is not a cycle") {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val z1 = cons(x, x)

            containsCycle(z1) shouldBe false
        }
    })
