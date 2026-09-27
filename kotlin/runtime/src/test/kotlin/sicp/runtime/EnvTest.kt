// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class EnvTest :
    FunSpec({
        test("define then lookup in one frame") {
            val env = Env.global()
            env.define("x", VInt(10))
            either { env.lookup("x") } shouldBe Either.Right(VInt(10))
        }

        test("lookup walks the chain outwards") {
            val global = Env.global()
            global.define("a", VInt(1))
            val inner = Env.child(global)
            inner.define("b", VInt(2))
            either { inner.lookup("a") } shouldBe Either.Right(VInt(1))
            either { inner.lookup("b") } shouldBe Either.Right(VInt(2))
        }

        test("an inner define shadows without touching the outer frame") {
            val global = Env.global()
            global.define("x", VInt(1))
            val inner = Env.child(global)
            inner.define("x", VInt(99))
            either { inner.lookup("x") } shouldBe Either.Right(VInt(99))
            either { global.lookup("x") } shouldBe Either.Right(VInt(1))
        }

        test("set mutates the nearest binding through every capture") {
            val global = Env.global()
            global.define("balance", VInt(100))
            val frame = Env.child(global)
            either { frame.set("balance", VInt(60)) }.isRight() shouldBe true
            either { global.lookup("balance") } shouldBe Either.Right(VInt(60))
        }

        test("set does not create bindings") {
            val global = Env.global()
            either { global.set("nope", VInt(1)) }.isLeft() shouldBe true
            either { global.lookup("nope") }.isLeft() shouldBe true
        }

        test("unbound lookup raises Unbound naming the symbol") {
            val env = Env.global()
            either { env.lookup("ghost") } shouldBe Either.Left(SchemeError.Unbound("ghost"))
        }

        test("extend binds names and a rest list") {
            val global = Env.global()
            either {
                val env = Env.extend(listOf("a"), listOf(VInt(1), VInt(2), VInt(3)), global, rest = "rest")
                env.lookup("a") shouldBe VInt(1)
                env.lookup("rest").toString() shouldBe "(2 3)"
            }.isRight() shouldBe true
        }

        test("extend keeps parameter order in frame iteration for lexical slots") {
            val global = Env.global()
            either {
                val names = listOf("p9", "p3", "p7", "p1", "p8", "p2", "p6", "p0", "p5", "p4")
                val env = Env.extend(names, names.map { VInt(1) }, global)
                env.frame.keys.toList() shouldBe names
                env.frame.entries
                    .elementAtOrNull(3)
                    ?.key shouldBe "p1"
            }.isRight() shouldBe true
        }

        test("extend rejects a short argument list") {
            val global = Env.global()
            either { Env.extend(listOf("a", "b"), listOf(VInt(1)), global) }.isLeft() shouldBe true
        }

        test("snapshot isolates define and set from the original chain") {
            val global = Env.global()
            global.define("x", VInt(1))
            val snap = global.snapshot()
            snap.define("y", VInt(2))
            either { snap.set("x", VInt(9)) }.isRight() shouldBe true
            either { global.lookup("x") } shouldBe Either.Right(VInt(1))
            either { global.lookup("y") }.isLeft() shouldBe true
            either { snap.lookup("x") } shouldBe Either.Right(VInt(9))
        }
    })
