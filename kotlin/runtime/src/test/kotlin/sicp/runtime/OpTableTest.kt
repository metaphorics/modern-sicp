// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeSameInstanceAs

public class OpTableTest :
    FunSpec({
        fun constant(n: Long): Op = { VInt(n) }

        test("put then get returns the installed handler") {
            val table = OpTable()
            table.put(Key.Sym("real-part"), Key.Sym("rectangular"), constant(1))
            val h = table.get(Key.Sym("real-part"), Key.Sym("rectangular"))
            either { h?.let { f -> f(listOf()) } } shouldBe Either.Right(VInt(1) as Value?)
        }

        test("a missing key returns the absent option") {
            val table = OpTable()
            table.get(Key.Sym("nope"), Key.Sym("polar")) shouldBe null
        }

        test("a later install overwrites an earlier one") {
            val table = OpTable()
            table.put(Key.Sym("add"), Key.Sym("scheme-number"), constant(1))
            table.put(Key.Sym("add"), Key.Sym("scheme-number"), constant(2))
            val h = table.get(Key.Sym("add"), Key.Sym("scheme-number"))
            either { h?.let { f -> f(listOf()) } } shouldBe Either.Right(VInt(2) as Value?)
        }

        test("tags and operations dispatch independently") {
            val table = OpTable()
            table.put(Key.Sym("imag-part"), Key.Sym("rectangular"), constant(1))
            table.put(Key.Sym("imag-part"), Key.Sym("polar"), constant(2))
            val rect = table.get(Key.Sym("imag-part"), Key.Sym("rectangular"))
            val polar = table.get(Key.Sym("imag-part"), Key.Sym("polar"))
            either { rect?.let { f -> f(listOf()) } } shouldBe Either.Right(VInt(1) as Value?)
            either { polar?.let { f -> f(listOf()) } } shouldBe Either.Right(VInt(2) as Value?)
        }

        test("handlers receive the argument slice") {
            val table = OpTable()
            table.put(Key.Sym("add"), Key.Sym("scheme-number")) { args ->
                val a = args.firstOrNull() ?: VNil
                val b = args.getOrNull(1) ?: VNil
                if (a is VInt && b is VInt) {
                    VInt(Math.addExact(a.n, b.n))
                } else {
                    raise(SchemeError.TypeMismatch("add: numbers"))
                }
            }
            val h = table.get(Key.Sym("add"), Key.Sym("scheme-number"))
            either { h?.let { f -> f(listOf(VInt(2), VInt(3))) } } shouldBe Either.Right(VInt(5) as Value?)
        }
    })
