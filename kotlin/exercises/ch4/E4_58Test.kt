// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.58

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_58Test :
    FunSpec({
        test("Exercise 4.58: the big-shot query").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            bigShotQuery() shouldBe listOf("MEASURE")
        }
    })
