// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.64

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_64Test :
    FunSpec({
        test("Exercise 4.64: Louis's swapped outranked-by").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            louisOutranked() shouldBe listOf("MEASURE")
        }
    })
