// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.32

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_32Test :
    FunSpec({
        test("Exercise 4.32: both slots delayed, the armed slot skipped then fired").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            lazyPairSlotsTranscript() shouldBe "7\nError: division by zero\n"
        }

        test("Exercise 4.32: the chapter-3 shape computes its head at construction").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            eagerConstructorTranscript() shouldBe "Error: division by zero\n"
        }

        test("Exercise 4.32: ones closes in one step under delayed construction").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            onesOneStepTranscript() shouldBe "1\n"
        }

        test("Exercise 4.32: the strict constructor reads the frame too early").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            strictOnesTranscript() shouldBe "Error: unbound variable: ones\n"
        }
    })
