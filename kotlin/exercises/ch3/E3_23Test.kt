// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.23

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.Symbol

public class E3_23Test :
    FunSpec({
        test("Exercise 3.23: constant-time ends, both directions").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val d = makeDeque()
            d.rearInsert(Symbol("a"))
            d.frontInsert(Symbol("z"))
            d.frontDeque() shouldBe Symbol("z")
            d.rearDeque() shouldBe Symbol("a")
            d.frontDelete()
            d.rearDelete()
            d.emptyDeque() shouldBe true
        }
    })
