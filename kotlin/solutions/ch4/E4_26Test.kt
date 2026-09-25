// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_26Test :
    FunSpec({
        test("Exercise 4.26: the derived unless skips the untouched arm") {
            unlessDerivedTranscript() shouldBe "42\n"
        }

        test("Exercise 4.26: the derived unless is syntax, not a value") {
            unlessDerivedValueUseTranscript() shouldBe "Error: unbound variable: unless\n"
        }

        test("Exercise 4.26: the lazy procedure skips the unchosen arm") {
            unlessLazyProcedureTranscript() shouldBe "42\n"
        }

        test("Exercise 4.26: the lazy procedure composes with map") {
            unlessLazyMappedTranscript() shouldBe "(42 7)\n"
        }

        test("Exercise 4.26: the lazy procedure composes with apply") {
            unlessLazyApplyTranscript() shouldBe "7\n"
        }
    })
