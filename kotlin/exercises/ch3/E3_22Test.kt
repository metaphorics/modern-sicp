// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.22

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.Symbol

public class E3_22Test :
    FunSpec({
        test("Exercise 3.22: the book's session through the closure queue").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val q1 = makeQueue()
            q1.insert(Symbol("a"))
            q1.insert(Symbol("b"))
            q1.delete()
            q1.items() shouldBe listOf(Symbol("b"))
        }
    })
