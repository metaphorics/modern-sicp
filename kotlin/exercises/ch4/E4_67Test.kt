// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.67

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_67Test :
    FunSpec({
        test("Exercise 4.67: the loop detector").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            loopDetectorDemos() shouldBe listOf("MEASURE")
        }
    })
