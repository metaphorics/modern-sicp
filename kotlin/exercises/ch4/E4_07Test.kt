// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.7

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_07Test :
    FunSpec({
        test("Exercise 4.7: the rewrite folds the bindings into nested lets").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            sequentialBindingsTranscript() shouldBe "12\n"
        }

        test("Exercise 4.7: each init sees the earlier bindings of the same let*").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            chainedBindingsTranscript() shouldBe "8\n"
        }

        test("Exercise 4.7: let* shadows from its first binding on").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            letStarShadowTranscript() shouldBe "1\n"
        }

        test("Exercise 4.7: a let* nested in another's body re-enters the clause").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            letStarNestedTranscript() shouldBe "2\n"
        }
    })
