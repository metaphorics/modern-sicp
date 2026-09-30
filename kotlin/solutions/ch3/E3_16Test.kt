// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.16

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.pair

public class E3_16Test :
    FunSpec({
        test("a chain of three unshared pairs counts correctly: 3") {
            val three = datumList(Symbol("a"), Symbol("b"), Symbol("c"))

            countPairs(three) shouldBe 3
        }

        test("three pairs with one shared down two paths count as 4") {
            val shared = datumList(Symbol("a"), Symbol("b")) as PairCell
            val says4 = pair(shared, shared.second)

            countPairs(says4) shouldBe 4
        }

        test("three pairs where each feeds both arms count as 7") {
            val q = pair(Symbol("a"), Empty)
            val p = pair(q, q)
            val says7 = pair(p, p)

            countPairs(says7) shouldBe 7
        }
    })
