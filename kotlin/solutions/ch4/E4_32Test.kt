// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.32

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_32Test :
    FunSpec({
        test("Exercise 4.32: both slots delayed, the armed slot skipped then fired") {
            lazyPairSlotsTranscript() shouldBe "7\nError: division by zero\n"
        }

        test("Exercise 4.32: the chapter-3 shape computes its head at construction") {
            eagerConstructorTranscript() shouldBe "Error: division by zero\n"
        }

        test("Exercise 4.32: ones closes in one step under delayed construction") {
            onesOneStepTranscript() shouldBe "1\n"
        }

        test("Exercise 4.32: the strict constructor reads the frame too early") {
            strictOnesTranscript() shouldBe "Error: unbound variable: ones\n"
        }
    })
