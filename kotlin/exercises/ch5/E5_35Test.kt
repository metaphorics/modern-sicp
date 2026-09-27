// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.35

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_35Test :
    FunSpec({
        test("Exercise 5.35: the seeded compilation replays Figure 5.18 statement for statement").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            figure5_18Compilation() shouldBe emptyList()
        }
    })
