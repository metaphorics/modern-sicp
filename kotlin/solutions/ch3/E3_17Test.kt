// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.17

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.pair

public class E3_17Test :
    FunSpec({
        test("every three-pair structure from 3.16 counts as 3") {
            val chain = datumList(Symbol("a"), Symbol("b"), Symbol("c"))
            val shared = datumList(Symbol("a"), Symbol("b")) as PairCell
            val says4 = pair(shared, shared.second)
            val q = pair(Symbol("a"), Empty)
            val p = pair(q, q)
            val says7 = pair(p, p)

            countDistinctPairs(chain) shouldBe 3
            countDistinctPairs(says4) shouldBe 3
            countDistinctPairs(says7) shouldBe 3
        }

        test("pair(x, x) contains three distinct pair objects") {
            val x = datumList(Symbol("a"), Symbol("b")) as PairCell
            val z1 = pair(x, x)

            countDistinctPairs(z1) shouldBe 3
        }

        test("each call gets a fresh seen collection") {
            val shared = datumList(Symbol("a"), Symbol("b")) as PairCell
            val says4 = pair(shared, shared.second)

            countDistinctPairs(says4) shouldBe 3
            countDistinctPairs(says4) shouldBe 3
        }
    })
