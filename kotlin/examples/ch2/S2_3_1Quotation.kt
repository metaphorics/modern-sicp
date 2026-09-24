// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.3.1

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.vlist

/**
 * The book's `memq`: finds `item` in the chain `x` by `eq?`, comparing
 * [VSym]s with `==` since a boxed symbol's `equals` is already structural
 * on its name. Returns [VNil]'s companion value `VBool(false)` when the
 * item is absent, exactly as the book's `cond` does; the search never
 * raises, since a well-formed proper list always terminates in [VNil].
 */
public fun memq(
    item: Value,
    x: Value,
): Value =
    when {
        x is VPair && x.car == item -> x
        x is VPair -> memq(item, x.cdr)
        else -> sicp.runtime.VBool(false)
    }

public class S2_3_1QuotationTest :
    FunSpec({
        test("vlist(a, b) builds a list of the two numbers' values") {
            val a = VInt(1L)
            val b = VInt(2L)
            vlist(a, b).toString() shouldBe "(1 2)"
        }
        test("vlist(VSym(...)) builds a list of the two symbols themselves") {
            vlist(VSym("a"), VSym("b")).toString() shouldBe "(a b)"
        }
        test("vlist can mix a symbol with a number's value") {
            val b = VInt(2L)
            vlist(VSym("a"), b).toString() shouldBe "(a 2)"
        }
        test("car and cdr read the parts of a Value list back out") {
            val abc = vlist(VSym("a"), VSym("b"), VSym("c"))
            (abc as VPair).car shouldBe VSym("a")
            abc.cdr.toString() shouldBe "(b c)"
        }
        test("memq('apple', ...) is false when apple is absent") {
            val set = vlist(VSym("pear"), VSym("banana"), VSym("prune"))
            memq(VSym("apple"), set).toString() shouldBe "#f"
        }
        test("memq('apple', ...) returns the sublist starting at the first match") {
            val xs =
                vlist(
                    VSym("x"),
                    vlist(VSym("apple"), VSym("sauce")),
                    VSym("y"),
                    VSym("apple"),
                    VSym("pear"),
                )
            memq(VSym("apple"), xs).toString() shouldBe "(apple pear)"
        }
    })
