// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.65a

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_65aTest :
    FunSpec({
        test("Exercise 4.65a: the deduplicated wheel listing").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            deduplicatedWheel() shouldBe listOf("MEASURE")
        }
    })
