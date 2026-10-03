// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.24: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_24Test :
    FunSpec({
        test("Exercise 4.24: re-analysis climbs with the runs, one analysis serves all") {
            analysisSavingsTranscript() shouldBe "7\n7\n7\n3\n3\n7\n7\n7\n1\n3\n"
        }

        test("Exercise 4.24: the analyzed engine agrees on answers and counters") {
            analyzedEngineTranscript() shouldBe "7\n7\n7\n1\n3\n"
        }
    })
