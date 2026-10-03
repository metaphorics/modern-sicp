// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Direct
import sicp.ch4.LazyModule

public class E426Test :
    FunSpec({
        test("Exercise 4.26: the derived unless answers without evaluating the unchosen arm") {
            unlessDerivedTranscript() shouldBe "42\n"
            outcomeText(Direct.run(UNLESS_PLANES_PROBE)) shouldBe "42\n0\nerror\n0\n"
        }

        test("Exercise 4.26: the derived name is not a value and the guard fires before argument effects") {
            unlessDerivedValueUseTranscript() shouldBe "error\n"
        }

        test("Exercise 4.26: the lazy procedure composes as an ordinary value") {
            unlessLazyProcedureTranscript() shouldBe "42\n"
            unlessLazyMappedTranscript() shouldBe "[42, 7]\n"
            unlessLazyApplyTranscript() shouldBe "7\n"
        }

        test("Exercise 4.26: the exceptional thunks never force in the lazy composition") {
            val run = LazyModule.run(UNLESS_LAZY_MAPPED_PROGRAM).fold({ e -> throw AssertionError(e.toString()) }, { it })
            run.result.error shouldBe null
            run.result.output shouldBe "[42, 7]\n"
        }
    })
