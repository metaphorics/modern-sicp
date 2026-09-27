// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.25

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.VSym

public class E3_25Test :
    FunSpec({
        test("values under one, two, and three keys live in one table") {
            val t = KeyListTable()
            t.insert(listOf(VSym("a")), VInt(1))
            t.insert(listOf(VSym("letters"), VSym("b")), VInt(98))
            t.insert(listOf(VSym("math"), VSym("+"), VSym("int")), VInt(43))
            t.lookup(listOf(VSym("a"))) shouldBe VInt(1)
            t.lookup(listOf(VSym("letters"), VSym("b"))) shouldBe VInt(98)
            t.lookup(listOf(VSym("math"), VSym("+"), VSym("int"))) shouldBe VInt(43)
        }

        test("a missing key at any depth is null") {
            val t = KeyListTable()
            t.insert(listOf(VSym("letters"), VSym("b")), VInt(98))
            t.lookup(listOf(VSym("letters"), VSym("z"))).shouldBeNull()
            t.lookup(listOf(VSym("math"), VSym("+"))).shouldBeNull()
            t.lookup(listOf(VSym("math"))).shouldBeNull()
        }

        test("inserting under an existing full key list overwrites the record") {
            val t = KeyListTable()
            t.insert(listOf(VSym("letters"), VSym("b")), VInt(98))
            t.insert(listOf(VSym("letters"), VSym("b")), VInt(99))
            t.lookup(listOf(VSym("letters"), VSym("b"))) shouldBe VInt(99)
        }

        test("a new first key splices a fresh subtable onto the backbone") {
            val t = KeyListTable()
            t.insert(listOf(VSym("letters"), VSym("a")), VInt(97))
            t.insert(listOf(VSym("math"), VSym("+")), VInt(43))
            t.lookup(listOf(VSym("letters"), VSym("a"))) shouldBe VInt(97)
            t.lookup(listOf(VSym("math"), VSym("+"))) shouldBe VInt(43)
        }

        test("the section's two-dimensional table is the two-key case") {
            val t = KeyListTable()
            t.insert(listOf(VSym("letters"), VSym("a")), VInt(97))
            t.insert(listOf(VSym("letters"), VSym("b")), VInt(98))
            t.lookup(listOf(VSym("letters"), VSym("a"))) shouldBe VInt(97)
            t.lookup(listOf(VSym("letters"), VSym("b"))) shouldBe VInt(98)
        }
    })
