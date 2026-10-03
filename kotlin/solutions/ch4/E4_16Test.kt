// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.16: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_16Test :
    FunSpec({
        test("Exercise 4.16: mutual recursion closes over the scan-out") {
            mutualRecursionTranscript() shouldBe "true\n"
        }

        test("Exercise 4.16: premature and unbound reads fail, the marker spells itself") {
            prematureReadTranscript() shouldBe "error\nerror\nunassigned\n"
        }
    })
