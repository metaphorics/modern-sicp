// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.67

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_67Test :
    FunSpec({
        test("Exercise 2.67: the sample message decodes to A D A B B C A").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_67() shouldBe listOf("A", "D", "A", "B", "B", "C", "A")
        }
    })
