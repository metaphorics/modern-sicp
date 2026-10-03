// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.examples

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeSameInstanceAs
import sicp.runtime.BindingFrame
import sicp.runtime.DatumError
import sicp.runtime.Symbol
import sicp.runtime.Whole
import sicp.runtime.pair

/** A captured `var` is one mutable cell shared by every closure that sees it. */
public fun makeCounter(start: Int): () -> Int {
    var n = start
    return {
        n += 1
        n
    }
}

/** The make-withdraw shape of section 3.1: the cell outlives the call. */
public fun makeWithdrawer(balance: Long): (Long) -> Long {
    var b = balance
    return { amount ->
        b -= amount
        b
    }
}

public class S0_5MutationTest :
    FunSpec({
        test("each counter is its own cell") {
            val c1 = makeCounter(0)
            val c2 = makeCounter(0)
            c1() shouldBe 1
            c1() shouldBe 2
            c2() shouldBe 1
        }
        test("a withdrawer remembers between calls") {
            val w = makeWithdrawer(100L)
            w(30L) shouldBe 70L
            w(20L) shouldBe 50L
        }
        test("a mutable pair cell preserves alias identity") {
            val p = pair(Whole(1L), Symbol("x"))
            val alias = p
            p.first = Whole(9L)
            alias.first shouldBe Whole(9L)
            alias shouldBeSameInstanceAs p
        }
        test("assign updates the nearest existing frame without creating a binding") {
            val global = BindingFrame.root()
            global.bind("x", Whole(1L))
            val inner = BindingFrame.child(global)
            inner.bind("x", Whole(2L))
            val nested = BindingFrame.child(inner)
            either { nested.read("x") } shouldBe Either.Right(Whole(2L))
            either { nested.assign("x", Whole(99L)) }.isRight() shouldBe true
            either { inner.read("x") } shouldBe Either.Right(Whole(99L))
            either { global.read("x") } shouldBe Either.Right(Whole(1L))
            either { nested.assign("missing", Whole(3L)) } shouldBe Either.Left(DatumError.BadDatum("missing"))
            either { nested.read("missing") } shouldBe Either.Left(DatumError.BadDatum("missing"))
        }
    })
