// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.9

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_09Test :
    FunSpec({
        test("Exercise 4.9: the while sum runs the body while the test holds").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            whileSumTranscript() shouldBe "15\n"
        }

        test("Exercise 4.9: a false test never runs the body").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            whileNeverRunsTranscript() shouldBe "0\n"
        }

        test("Exercise 4.9: the until product and its counter land together").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            untilProductTranscript() shouldBe "95040\n13\n"
        }

        test("Exercise 4.9: a holding test never runs the until body").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            untilNeverRunsTranscript() shouldBe "0\n"
        }

        test("Exercise 4.9: loops nest, each block holding its own loop state").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            nestedLoopsTranscript() shouldBe "6\n"
        }
    })
