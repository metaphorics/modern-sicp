// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.21

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.Symbol

public class E3_21Test :
    FunSpec({
        test("Exercise 3.21: the view contains queue items and its rear datum").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val q = Queue()
            q.insert(Symbol("a"))
            q.insert(Symbol("b"))
            q.insert(Symbol("c"))
            q.insert(Symbol("d"))

            q.items() shouldBe listOf(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d"))
            benView(q) shouldBe (listOf(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d")) to Symbol("d"))
            q.rearCell()?.first shouldBe Symbol("d")
        }
    })
