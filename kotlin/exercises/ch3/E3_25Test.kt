// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.25

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.VSym

public class E3_25Test :
    FunSpec({
        test("Exercise 3.25: values under one, two, and three keys in one table").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val t = KeyListTable()
            t.insert(listOf(VSym("a")), VInt(1))
            t.insert(listOf(VSym("letters"), VSym("b")), VInt(98))
            t.insert(listOf(VSym("math"), VSym("+"), VSym("int")), VInt(43))
            t.lookup(listOf(VSym("letters"), VSym("b"))) shouldBe VInt(98)
            t.lookup(listOf(VSym("math"), VSym("+"), VSym("int"))) shouldBe VInt(43)
            t.lookup(listOf(VSym("a"))) shouldBe VInt(1)
        }
    })
