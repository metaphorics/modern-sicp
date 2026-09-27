// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.4.4.2, the evaluator: `qeval`'s data-directed
// dispatch, the series `and`, the interleaved `or`, the `not` and
// `lisp-value` filters, and the `always-true` handler for bodyless rules.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class S4_4_4EvaluatorTest :
    FunSpec({
        val system = microshaftSystem()

        test("qeval dispatches and through the table") {
            answersOf(system, "(and (supervisor ?x (Bitdiddle Ben)) (not (job ?x (computer technician))))") shouldBe
                listOf(
                    "(and (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (not (job (Hacker Alyssa P) (computer technician))))",
                    "(and (supervisor (Fect Cy D) (Bitdiddle Ben)) (not (job (Fect Cy D) (computer technician))))",
                )
        }

        test("the or dispatch interleaves, appending would starve") {
            answersOf(system, "(or (supervisor ?x (Hacker Alyssa P)) (supervisor ?x (Scrooge Eben)))") shouldBe
                listOf(
                    "(or (supervisor (Reasoner Louis) (Hacker Alyssa P)) (supervisor (Reasoner Louis) (Scrooge Eben)))",
                    "(or (supervisor (Cratchet Robert) (Hacker Alyssa P)) (supervisor (Cratchet Robert) (Scrooge Eben)))",
                )
        }

        test("always-true passes every frame for a bodyless rule") {
            answersOf(system, "(same (Bitdiddle Ben) (Bitdiddle Ben))") shouldBe
                listOf("(same (Bitdiddle Ben) (Bitdiddle Ben))")
            answersOf(system, "(same (Bitdiddle Ben) (Fect Cy D))") shouldBe emptyList()
        }
    })
