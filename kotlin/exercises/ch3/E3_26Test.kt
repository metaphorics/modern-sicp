// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.26

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.VSym

public class E3_26Test :
    FunSpec({
        test("Exercise 3.26: ordered records make lookup a tree walk").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val t = TreeTable()
            t.insert(VSym("d"), VInt(4))
            t.insert(VSym("b"), VInt(2))
            t.insert(VSym("f"), VInt(6))
            t.lookup(VSym("b")) shouldBe VInt(2)
            t.lookup(VSym("f")) shouldBe VInt(6)
            t.lookup(VSym("d")) shouldBe VInt(4)
            t.lookup(VSym("e")).shouldBeNull()
        }
    })
