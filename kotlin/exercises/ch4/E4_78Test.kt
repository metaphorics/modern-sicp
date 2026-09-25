// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.78

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_78Test :
    FunSpec({
        test("Exercise 4.78: the amb port").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ambQueryDemos() shouldBe listOf("MEASURE")
        }
    })
