// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.raise.either
import io.kotest.assertions.assertSoftly
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeSameInstanceAs
import kotlinx.collections.immutable.persistentListOf

public class ValueTest :
    FunSpec({
        test("vlist builds a proper list and listItems reads it back") {
            val xs = vlist(VInt(1), VInt(2), VInt(3))
            xs.toString() shouldBe "(1 2 3)"
            either { listItems(xs) } shouldBe
                arrow.core.Either.Right(persistentListOf(VInt(1), VInt(2), VInt(3)))
        }

        test("dotted tails print with the dot and reject listItems") {
            val p = cons(VInt(1), VInt(2))
            p.toString() shouldBe "(1 . 2)"
            either { listItems(p) }.isLeft() shouldBe true
        }

        test("setCar and setCdr are visible through every alias") {
            val p = cons(VInt(1), VInt(2))
            val alias = p
            p.setCar(VInt(9))
            alias.car shouldBe VInt(9)
            p.setCdr(vlist(VInt(3)))
            p.toString() shouldBe "(9 3)"
        }

        test("eq is identity on pairs, equalv compares structure") {
            val a = cons(VInt(1), VInt(2))
            val b = cons(VInt(1), VInt(2))
            assertSoftly {
                (a == b) shouldBe false
                equalv(a, b) shouldBe true
                (a == a) shouldBe true
            }
        }

        test("car and cdr raise TypeMismatch on non-pairs") {
            either { car(VInt(3)) } shouldBe
                arrow.core.Either.Left(SchemeError.TypeMismatch("car of a non-pair: 3"))
            either { cdr(VSym("x")) }.isLeft() shouldBe true
        }

        test("forceIt memoizes a VThunk and re-runs a VThunkNoMemo") {
            var calls = 0
            val env = Env.global()
            val eval: arrow.core.raise.Raise<SchemeError>.(Expr, Env) -> Value = { _, _ ->
                calls++
                VInt(7)
            }
            val memo = delayIt(LitE(VInt(0)), env)
            val noMemo = VThunkNoMemo(LitE(VInt(0)), env)
            either {
                forceIt(memo, eval) shouldBe VInt(7)
                forceIt(memo, eval) shouldBe VInt(7)
                calls shouldBe 1
                forceIt(noMemo, eval)
                forceIt(noMemo, eval)
                calls shouldBe 3
            }
        }

        test("a failed force leaves the thunk delayed") {
            var calls = 0
            val env = Env.global()
            val eval: arrow.core.raise.Raise<SchemeError>.(Expr, Env) -> Value = { _, _ ->
                calls++
                if (calls == 1) raise(SchemeError.DivisionByZero)
                VInt(5)
            }
            val t = delayIt(LitE(VInt(0)), env)
            either { forceIt(t, eval) }.isLeft() shouldBe true
            either { forceIt(t, eval) } shouldBe arrow.core.Either.Right(VInt(5))
        }

        test("callPrimitive applies the body and rejects non-primitives") {
            val plus =
                VPrimitive("+") { args ->
                    val (a, b) = args
                    VInt((a as VInt).n + (b as VInt).n)
                }
            either { callPrimitive(plus, listOf(VInt(2), VInt(3))) } shouldBe
                arrow.core.Either.Right(VInt(5))
            either { callPrimitive(VInt(1), listOf()) }.isLeft() shouldBe true
        }

        test("isTrue rejects only #f") {
            assertSoftly {
                isTrue(VBool(false)) shouldBe false
                isTrue(VBool(true)) shouldBe true
                isTrue(VNil) shouldBe true
                isTrue(VInt(0)) shouldBe true
            }
        }

        test("toString renders the book's surface syntax") {
            assertSoftly {
                VBool(true).toString() shouldBe "#t"
                VStr("a\"b").toString() shouldBe "\"a\\\"b\""
                VTagged("rectangular", VNil).toString() shouldBe "(rectangular)"
                VPrimitive("+") { VNil }.toString() shouldBe "#[primitive +]"
                VProc(persistentListOf("x"), null, persistentListOf(), Env.global())
                    .toString() shouldBe "#[compound-procedure]"
                VCompiledProc("entry", persistentListOf(), Env.global())
                    .toString() shouldBe "#[compiled-procedure entry]"
            }
        }

        test("keyOf projects keyable shapes and rejects the rest") {
            either { keyOf(VSym("rectangular")) } shouldBe arrow.core.Either.Right(Key.Sym("rectangular"))
            either { keyOf(vlist(VSym("a"), VInt(1))) } shouldBe
                arrow.core.Either.Right(Key.Pair(Key.Sym("a"), Key.Pair(Key.Int(1), Key.Nil)))
            either { keyOf(VReal(1.5)) }.isLeft() shouldBe true
            either { keyOf(VBool(true)) }.isLeft() shouldBe true
        }
    })
