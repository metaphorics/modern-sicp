// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.69

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_69Test :
    FunSpec({
        test("Exercise 4.69: the greats queries").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            greatsQueries() shouldBe listOf("MEASURE")
        }
    })
