// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.04

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_04Test :
    FunSpec({
        test("Exercise 5.04: the two exponentiation machines").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            exptMachineRuns() shouldBe
                listOf(
                    "1024",
                    "243",
                    "1024",
                    "243",
                )
        }
    })
