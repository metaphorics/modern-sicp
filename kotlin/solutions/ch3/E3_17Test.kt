// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.17

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.cons
import sicp.runtime.vlist

public class E3_17Test :
    FunSpec({
        test("every three-pair structure from 3.16 counts as 3") {
            val chain = vlist(VSym("a"), VSym("b"), VSym("c"))
            val shared = vlist(VSym("a"), VSym("b")) as VPair
            val says4 = cons(shared, shared.cdr)
            val q = cons(VSym("a"), VNil)
            val p = cons(q, q)
            val says7 = cons(p, p)

            countDistinctPairs(chain) shouldBe 3
            countDistinctPairs(says4) shouldBe 3
            countDistinctPairs(says7) shouldBe 3
        }

        test("cons(x, x) holds three distinct pairs, not four") {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val z1 = cons(x, x)

            countDistinctPairs(z1) shouldBe 3
        }

        test("a structure can be traversed twice: the seen list does not leak") {
            val shared = vlist(VSym("a"), VSym("b")) as VPair
            val says4 = cons(shared, shared.cdr)

            countDistinctPairs(says4) shouldBe 3
            countDistinctPairs(says4) shouldBe 3
        }
    })
