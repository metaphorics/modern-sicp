// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.18

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.pair

public class E3_18Test :
    FunSpec({
        test("the cycle made in 3.13 is detected") {
            val cycle = makeCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c")) as PairCell)

            containsCycle(cycle) shouldBe true
        }

        test("finite pair chains are reported acyclic") {
            containsCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c"))) shouldBe false
            containsCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d"), Symbol("e"))) shouldBe false
        }

        test("sharing a pair through a first field is not a cycle in the second-field chain") {
            val x = datumList(Symbol("a"), Symbol("b")) as PairCell
            val shared = pair(x, x)

            containsCycle(shared) shouldBe false
        }
    })
