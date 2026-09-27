// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.73

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_73Test :
    FunSpec({
        test("Exercise 4.73: the flatten-stream delay").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            flattenDelayDebate() shouldBe listOf("MEASURE")
        }
    })
