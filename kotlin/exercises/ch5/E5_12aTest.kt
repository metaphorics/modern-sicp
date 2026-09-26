// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.12a

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_12aTest :
    FunSpec({
        test("Exercise 5.12a: the per-type census line of the summary").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            gcdMachineCensus() shouldBe "MEASURE"
        }
    })
