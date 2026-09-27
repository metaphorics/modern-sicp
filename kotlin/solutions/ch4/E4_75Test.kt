// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.75

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_75Test :
    FunSpec({
        test("Exercise 4.75: the unique special form") {
            uniqueDemos() shouldBe
                listOf(
                    "query: (unique (job ?x (computer wizard)))",
                    "(unique (job ?x (computer wizard)))",
                    "query: (unique (job ?x (computer programmer)))",
                    "query: (and (job ?x ?j) (unique (job ?anyone ?j)))",
                    "answers=7",
                    "(and (job (Bitdiddle Ben) (computer wizard)) (unique (job ?anyone (computer wizard))))",
                    "(and (job (Tweakit Lem E) (computer technician)) (unique (job ?anyone (computer technician))))",
                    "(and (job (Reasoner Louis) (computer programmer trainee)) (unique (job ?anyone (computer programmer trainee))))",
                    "(and (job (Warbucks Oliver) (administration big wheel)) (unique (job ?anyone (administration big wheel))))",
                    "(and (job (Scrooge Eben) (accounting chief accountant)) (unique (job ?anyone (accounting chief accountant))))",
                    "(and (job (Cratchet Robert) (accounting scrivener)) (unique (job ?anyone (accounting scrivener))))",
                    "(and (job (Aull DeWitt) (administration secretary)) (unique (job ?anyone (administration secretary))))",
                    "query: (and (supervisor ?person ?boss) (unique (supervisor ?underling ?boss))) -- answers=2",
                    "(and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (unique (supervisor ?underling (Hacker Alyssa P))))",
                    "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (unique (supervisor ?underling (Scrooge Eben))))",
                )
        }
    })
