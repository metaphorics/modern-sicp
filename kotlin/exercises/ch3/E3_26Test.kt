// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.26

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.Symbol
import sicp.runtime.Whole

public class E3_26Test :
    FunSpec({
        test("Exercise 3.26: ordered records make lookup a tree walk").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val t = TreeTable()
            t.insert(Symbol("d"), Whole(4))
            t.insert(Symbol("b"), Whole(2))
            t.insert(Symbol("f"), Whole(6))
            t.lookup(Symbol("b")) shouldBe Whole(2)
            t.lookup(Symbol("f")) shouldBe Whole(6)
            t.lookup(Symbol("d")) shouldBe Whole(4)
            t.lookup(Symbol("e")).shouldBeNull()
        }
    })
