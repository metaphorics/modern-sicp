// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.25

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.Symbol
import sicp.runtime.Whole

public class E3_25Test :
    FunSpec({
        test("Exercise 3.25: values under one, two, and three keys in one table").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val t = KeyListTable()
            t.insert(listOf(Symbol("a")), Whole(1))
            t.insert(listOf(Symbol("letters"), Symbol("b")), Whole(98))
            t.insert(listOf(Symbol("math"), Symbol("+"), Symbol("int")), Whole(43))
            t.lookup(listOf(Symbol("letters"), Symbol("b"))) shouldBe Whole(98)
            t.lookup(listOf(Symbol("math"), Symbol("+"), Symbol("int"))) shouldBe Whole(43)
            t.lookup(listOf(Symbol("a"))) shouldBe Whole(1)
        }
    })
