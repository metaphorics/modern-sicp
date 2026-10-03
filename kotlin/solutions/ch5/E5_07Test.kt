// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_07

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_07Test :
    FunSpec({
        test("Exercise 5.7: the 5.4 expt machines run on the simulator against the host oracle") {
            simulatedExptRuns() shouldBe
                listOf(
                    "recursive expt(2, 10) = 1024 (host 1024)",
                    "recursive expt(3, 5) = 243 (host 243)",
                    "iterative expt(2, 10) = 1024 (host 1024)",
                    "iterative expt(3, 5) = 243 (host 243)",
                )
        }
        test("Exercise 5.7: the book's gcd machine of 5.2 answers 2") {
            runGcd(206, 40) shouldBe 2L
        }
    })
