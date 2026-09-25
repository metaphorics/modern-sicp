// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.05

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_05Test :
    FunSpec({
        test("Exercise 5.05: hand simulation of the factorial and fib machines").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            handSimulationTraces() shouldBe listOf("MEASURE")
        }
    })
