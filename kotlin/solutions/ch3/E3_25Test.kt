// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.25

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.Symbol
import sicp.runtime.Whole

public class E3_25Test :
    FunSpec({
        test("values under one, two, and three keys live in one table") {
            val t = KeyListTable()
            t.insert(listOf(Symbol("a")), Whole(1))
            t.insert(listOf(Symbol("letters"), Symbol("b")), Whole(98))
            t.insert(listOf(Symbol("math"), Symbol("+"), Symbol("int")), Whole(43))
            t.lookup(listOf(Symbol("a"))) shouldBe Whole(1)
            t.lookup(listOf(Symbol("letters"), Symbol("b"))) shouldBe Whole(98)
            t.lookup(listOf(Symbol("math"), Symbol("+"), Symbol("int"))) shouldBe Whole(43)
        }

        test("a missing key at any depth is null") {
            val t = KeyListTable()
            t.insert(listOf(Symbol("letters"), Symbol("b")), Whole(98))
            t.lookup(listOf(Symbol("letters"), Symbol("z"))).shouldBeNull()
            t.lookup(listOf(Symbol("math"), Symbol("+"))).shouldBeNull()
            t.lookup(listOf(Symbol("math"))).shouldBeNull()
        }

        test("inserting under an existing full key list overwrites the record") {
            val t = KeyListTable()
            t.insert(listOf(Symbol("letters"), Symbol("b")), Whole(98))
            t.insert(listOf(Symbol("letters"), Symbol("b")), Whole(99))
            t.lookup(listOf(Symbol("letters"), Symbol("b"))) shouldBe Whole(99)
        }

        test("a new first key splices a fresh subtable onto the backbone") {
            val t = KeyListTable()
            t.insert(listOf(Symbol("letters"), Symbol("a")), Whole(97))
            t.insert(listOf(Symbol("math"), Symbol("+")), Whole(43))
            t.lookup(listOf(Symbol("letters"), Symbol("a"))) shouldBe Whole(97)
            t.lookup(listOf(Symbol("math"), Symbol("+"))) shouldBe Whole(43)
        }

        test("the section's two-dimensional table is the two-key case") {
            val t = KeyListTable()
            t.insert(listOf(Symbol("letters"), Symbol("a")), Whole(97))
            t.insert(listOf(Symbol("letters"), Symbol("b")), Whole(98))
            t.lookup(listOf(Symbol("letters"), Symbol("a"))) shouldBe Whole(97)
            t.lookup(listOf(Symbol("letters"), Symbol("b"))) shouldBe Whole(98)
        }
    })
