// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.12

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.Env

public class E4_12Test :
    FunSpec({
        test("Exercise 4.12: define, lookup, and set! run through the scan abstractions").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            scannedTranscript() shouldBe "2\n10\n10\n"
        }

        test("Exercise 4.12: a binding that died with its call frame is unbound at top level").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            scannedFreshFrameTranscript() shouldBe "2\nError: unbound variable: z\n"
        }

        test("Exercise 4.12: frameScan reads one frame, envScan walks the chain").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val env = Env.global()
            frameScan("z", env) shouldBe null
            envScan("z", env) shouldBe null
        }

        test("Exercise 4.12: extend-environment keeps the arity contract on the scans").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            scannedArityTranscript() shouldBe "Error: extend: wrong number of arguments, expected 2, got 1\n"
        }
    })
