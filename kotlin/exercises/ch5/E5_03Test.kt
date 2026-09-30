// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.03

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_03Test :
    FunSpec({
        test("Exercise 5.03: the sqrt machine in two stages").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            sqrtMachineTranscripts() shouldBe
                listOf(
                    "1.4142156862745097",
                    "1.4142156862745097",
                    "3.00009155413138",
                    "3.00009155413138",
                )
        }
    })
