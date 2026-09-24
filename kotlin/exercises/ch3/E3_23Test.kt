// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.23

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.VSym

public class E3_23Test :
    FunSpec({
        test("Exercise 3.23: constant-time ends, both directions").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val d = makeDeque()
            d.rearInsert(VSym("a"))
            d.frontInsert(VSym("z"))
            d.frontDeque() shouldBe VSym("z")
            d.rearDeque() shouldBe VSym("a")
            d.frontDelete()
            d.rearDelete()
            d.emptyDeque() shouldBe true
        }
    })
