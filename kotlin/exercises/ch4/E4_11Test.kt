// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.11

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_11Test :
    FunSpec({
        test("Exercise 4.11: define, lookup, and set! run over alist frames").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            alistTranscript() shouldBe "2\n10\n10\n"
        }

        test("Exercise 4.11: a binding that died with its call frame is unbound at top level").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            alistFreshFrameTranscript() shouldBe "2\nError: unbound variable: z\n"
        }

        test("Exercise 4.11: extend-environment keeps the arity contract on alist frames").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            alistArityTranscript() shouldBe "Error: extend: wrong number of arguments, expected 2, got 1\n"
        }
    })
