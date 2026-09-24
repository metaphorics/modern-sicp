// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.12

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.vlist

public class E3_12Test :
    FunSpec({
        test("append's z prints (a b c d) and x keeps its own two cells") {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val y = vlist(VSym("c"), VSym("d")) as VPair

            val z = append(x, y)

            z.toString() shouldBe "(a b c d)"
            x.cdr.toString() shouldBe "(b)"
        }

        test("appendBang mutates the shared tail, so x.cdr is now (b c d)") {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val y = vlist(VSym("c"), VSym("d")) as VPair
            append(x, y)

            val w = appendBang(x, y)

            w.toString() shouldBe "(a b c d)"
            x.cdr.toString() shouldBe "(b c d)"
        }

        test("appendBang splices rather than copies: w is x and ends at y's last pair") {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val y = vlist(VSym("c"), VSym("d")) as VPair

            val w = appendBang(x, y)

            (w === x) shouldBe true
            (lastPair(w) === lastPair(y)) shouldBe true
        }

        test("lastPair returns the final pair, itself for a one-pair chain") {
            val x = vlist(VSym("a"), VSym("b"), VSym("c")) as VPair
            val second = x.cdr as VPair
            val single = vlist(VSym("a")) as VPair

            (lastPair(x) === second.cdr) shouldBe true
            (lastPair(single) === single) shouldBe true
        }
    })
