// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.16

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.cons
import sicp.runtime.vlist

public class E3_16Test :
    FunSpec({
        test("a chain of three unshared pairs counts correctly: 3") {
            val three = vlist(VSym("a"), VSym("b"), VSym("c"))

            countPairs(three) shouldBe 3
        }

        test("three pairs with one shared down two paths count as 4") {
            val shared = vlist(VSym("a"), VSym("b")) as VPair
            val says4 = cons(shared, shared.cdr)

            countPairs(says4) shouldBe 4
        }

        test("three pairs where each feeds both arms count as 7") {
            val q = cons(VSym("a"), VNil)
            val p = cons(q, q)
            val says7 = cons(p, p)

            countPairs(says7) shouldBe 7
        }
    })
