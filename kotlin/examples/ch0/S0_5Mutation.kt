// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.examples

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeSameInstanceAs
import sicp.runtime.Env
import sicp.runtime.SchemeError
import sicp.runtime.VInt
import sicp.runtime.VSym
import sicp.runtime.cons
import sicp.runtime.setCar

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
        test("setCar mutates the one cell every alias sees") {
            val p = cons(VInt(1), VSym("x"))
            val alias = p
            p.setCar(VInt(9))
            alias.car shouldBe VInt(9)
            alias shouldBeSameInstanceAs p
        }
        test("set mutates the nearest frame holding the name and never creates one") {
            val global = Env.global()
            val inner = Env.child(global)
            inner.define("x", VInt(1))
            either { inner.lookup("x") } shouldBe Either.Right(VInt(1))
            either { inner.set("x", VInt(99)) }.isRight() shouldBe true
            either { inner.lookup("x") } shouldBe Either.Right(VInt(99))
            either { global.lookup("x") } shouldBe Either.Left(SchemeError.Unbound("x"))
        }
    })
