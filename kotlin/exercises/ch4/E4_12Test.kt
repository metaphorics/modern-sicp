// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.12

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_12Test :
    FunSpec({
        test("Exercise 4.12: define, lookup, and assignment run over the scans").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            scannedTranscript() shouldBe "2\n10\n10\n"
        }

        test("Exercise 4.12: a binding that died with its call frame is gone at top level").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            scannedFreshFrameTranscript() shouldBe "2\nnull\n"
        }

        test("Exercise 4.12: the arity contract holds on the scanned frames").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            scannedArityTranscript() shouldBe "null\n"
        }
    })
