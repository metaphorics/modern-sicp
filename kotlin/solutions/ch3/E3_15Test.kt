// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.15

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.cons
import sicp.runtime.vlist

public class E3_15Test :
    FunSpec({
        test("z1's car and cdr are one shared pair, so one setCar shows twice") {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val z1 = cons(x, x)

            (z1.car === z1.cdr) shouldBe true
            z1.toString() shouldBe "((a b) a b)"

            setToWow(z1).toString() shouldBe "((wow b) wow b)"
            z1.toString() shouldBe "((wow b) wow b)"
        }

        test("z2's two (a b) lists are distinct pairs, so only the car changes") {
            val z2 = cons(vlist(VSym("a"), VSym("b")), vlist(VSym("a"), VSym("b")))

            (z2.car === z2.cdr) shouldBe false
            z2.toString() shouldBe "((a b) a b)"

            setToWow(z2).toString() shouldBe "((wow b) a b)"
            (z2.cdr as VPair).toString() shouldBe "(a b)"
        }
    })
