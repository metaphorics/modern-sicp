// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.21

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.VSym

public class E3_21Test :
    FunSpec({
        test("Exercise 3.21: the raw pair of pointers Ben's interpreter printed").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val q1 = Queue()
            q1.insert(VSym("a"))
            q1.insert(VSym("b"))
            q1.insert(VSym("c"))
            q1.insert(VSym("d"))
            q1.printQueue() shouldBe "(a b c d)"
            benView(q1) shouldBe "((a b c d) d)"
        }
    })
