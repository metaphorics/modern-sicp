// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.07

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_07Test :
    FunSpec({
        test("Exercise 5.07: the 5.4 expt machines run on the simulator").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            simulatedExptRuns() shouldBe
                listOf(
                    "recursive expt(2, 10) = 1024 (host 1024)",
                    "recursive expt(3, 5) = 243 (host 243)",
                    "iterative expt(2, 10) = 1024 (host 1024)",
                    "iterative expt(3, 5) = 243 (host 243)",
                )
        }
    })
