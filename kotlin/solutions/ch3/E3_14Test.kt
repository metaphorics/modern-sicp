// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.14

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.vlist

public class E3_14Test :
    FunSpec({
        test("the book's session: v prints (a) and w prints (d c b a)") {
            val v = vlist(VSym("a"), VSym("b"), VSym("c"), VSym("d")) as VPair

            val w = mystery(v)

            v.toString() shouldBe "(a)"
            w.toString() shouldBe "(d c b a)"
        }

        test("the reversal is in place: w's cells are v's original cells reversed") {
            val v = vlist(VSym("a"), VSym("b"), VSym("c"), VSym("d")) as VPair
            val second = v.cdr as VPair
            val third = second.cdr as VPair
            val last = third.cdr as VPair

            val w = mystery(v)

            (w === last) shouldBe true
            (w.cdr === third) shouldBe true
            ((w.cdr as VPair).cdr === second) shouldBe true
            (((w.cdr as VPair).cdr as VPair).cdr === v) shouldBe true
        }

        test("a one-pair chain is its own reverse") {
            val v = vlist(VSym("a")) as VPair

            val w = mystery(v)

            (w === v) shouldBe true
            v.toString() shouldBe "(a)"
        }
    })
