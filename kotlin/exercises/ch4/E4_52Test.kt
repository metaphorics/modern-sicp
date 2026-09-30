// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.52

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_52Test :
    FunSpec({
        test("Exercise 4.52: the exhausted first body hands over to the fallback").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ifFailAllOddTranscript() shouldBe "all-odd\n"
        }

        test("Exercise 4.52: the fallback waits for the first body's answers").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ifFailEightTranscript() shouldBe "8\nall-odd\n"
        }
    })
