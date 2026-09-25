// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_26Test :
    FunSpec({
        test("Exercise 4.26: the derived unless skips the untouched arm").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            unlessDerivedTranscript() shouldBe "42\n"
        }

        test("Exercise 4.26: the derived unless is syntax, not a value").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            unlessDerivedValueUseTranscript() shouldBe "Error: unbound variable: unless\n"
        }

        test("Exercise 4.26: the lazy procedure skips the unchosen arm").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            unlessLazyProcedureTranscript() shouldBe "42\n"
        }

        test("Exercise 4.26: the lazy procedure composes with map").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            unlessLazyMappedTranscript() shouldBe "(42 7)\n"
        }

        test("Exercise 4.26: the lazy procedure composes with apply").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            unlessLazyApplyTranscript() shouldBe "7\n"
        }
    })
