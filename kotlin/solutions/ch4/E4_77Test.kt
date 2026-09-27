// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.77

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_77Test :
    FunSpec({
        test("Exercise 4.77: the delayed filters") {
            delayedFilterDemos() shouldBe
                listOf(
                    "query: (and (not (job ?x (computer programmer))) (supervisor ?x ?y))",
                    "(and (not (job (Tweakit Lem E) (computer programmer))) (supervisor (Tweakit Lem E) (Bitdiddle Ben)))",
                    "(and (not (job (Reasoner Louis) (computer programmer))) (supervisor (Reasoner Louis) (Hacker Alyssa P)))",
                    "(and (not (job (Bitdiddle Ben) (computer programmer))) (supervisor (Bitdiddle Ben) (Warbucks Oliver)))",
                    "(and (not (job (Scrooge Eben) (computer programmer))) (supervisor (Scrooge Eben) (Warbucks Oliver)))",
                    "(and (not (job (Cratchet Robert) (computer programmer))) (supervisor (Cratchet Robert) (Scrooge Eben)))",
                    "(and (not (job (Aull DeWitt) (computer programmer))) (supervisor (Aull DeWitt) (Warbucks Oliver)))",
                    "deferred=1 fulfilled=8 unresolved=0",
                    "query: (and (lisp-value > ?amount 30000) (salary ?who ?amount))",
                    "the naive order diverges in this engine before answering",
                    "deferred=2 fulfilled=9 unresolved=0",
                    "query: (and (salary ?who ?amount) (lisp-value > ?amount 30000)) -- answers=1",
                    "the naive order diverges in this engine before answering",
                    "deferred=2 fulfilled=9 unresolved=0",
                )
        }
    })
