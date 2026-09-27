// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.19

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.cons
import sicp.runtime.setCdr
import sicp.runtime.vlist

public class E3_19Test :
    FunSpec({
        test("the 3.13 cycle is caught in constant space") {
            val z = makeCycle(vlist(VSym("a"), VSym("b"), VSym("c")) as VPair)

            containsCycleConstantSpace(z) shouldBe true
        }

        test("a cell whose cdr is itself is caught too") {
            val selfLoop = cons(VSym("a"), sicp.runtime.VNil)
            selfLoop.setCdr(selfLoop)

            containsCycleConstantSpace(selfLoop) shouldBe true
        }

        test("chains of even and odd length are reported acyclic") {
            containsCycleConstantSpace(vlist(VSym("a"), VSym("b"))) shouldBe false
            containsCycleConstantSpace(vlist(VSym("a"), VSym("b"), VSym("c"))) shouldBe false
            containsCycleConstantSpace(cons(VSym("a"), sicp.runtime.VNil)) shouldBe false
        }

        test("agrees with the 3.18 remembered-pairs detector on both answers") {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val z1 = cons(x, x)
            val z = makeCycle(vlist(VSym("a"), VSym("b"), VSym("c")) as VPair)

            (containsCycleConstantSpace(z1) == containsCycle(z1)) shouldBe true
            (containsCycleConstantSpace(z) == containsCycle(z)) shouldBe true
        }
    })
