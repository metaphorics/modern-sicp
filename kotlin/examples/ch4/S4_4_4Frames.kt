// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.4.8, frames and bindings: the persistent map,
// the unmarked-inhabitant rule -- failure is the option type, never a
// sentinel binding -- and the book's `extend`/`binding-in-frame` pair.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Frame
import sicp.ch4.bindingInFrame
import sicp.ch4.extend
import sicp.ch4.patternVar

public class S4_4_4FramesTest :
    FunSpec({
        test("the empty frame binds nothing") {
            bindingInFrame(patternVar("x"), Frame.Empty) shouldBe null
        }

        test("extend adds one binding without touching the base") {
            val base = Frame.Empty
            val f1 = extend(patternVar("x"), readQuery("(a b)"), base)
            val f2 = extend(patternVar("y"), readQuery("c"), f1)
            bindingInFrame(patternVar("x"), f2).toString() shouldBe "(a b)"
            bindingInFrame(patternVar("y"), f2).toString() shouldBe "c"
            bindingInFrame(patternVar("x"), base) shouldBe null
            bindingInFrame(patternVar("x"), f1).toString() shouldBe "(a b)"
        }

        test("a re-extend replaces, per the persistent map") {
            val f = extend(patternVar("x"), readQuery("1"), Frame.Empty).extended(patternVar("x"), readQuery("2"))
            bindingInFrame(patternVar("x"), f).toString() shouldBe "2"
        }

        test("frames with equal bindings are equal") {
            val a = extend(patternVar("x"), readQuery("1"), Frame.Empty)
            val b = extend(patternVar("x"), readQuery("1"), Frame.Empty)
            a shouldBe b
        }
    })
