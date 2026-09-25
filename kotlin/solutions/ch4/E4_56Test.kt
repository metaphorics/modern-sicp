// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.56

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_56Test :
    FunSpec({
        test("Exercise 4.56: the three compound queries") {
            compoundQueries() shouldBe
                listOf(
                    "query: (and (supervisor ?person (Bitdiddle Ben)) (address ?person ?where))",
                    "(and (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (address (Hacker Alyssa P) (Cambridge (Mass Ave) 78)))",
                    "(and (supervisor (Fect Cy D) (Bitdiddle Ben)) (address (Fect Cy D) (Cambridge (Ames Street) 3)))",
                    "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (address (Tweakit Lem E) (Boston (Bay State Road) 22)))",
                    "query: (and (salary ?person ?amount) (salary (Bitdiddle Ben) ?ben-amount) (lisp-value < ?amount ?ben-amount))",
                    "(and (salary (Hacker Alyssa P) 40000) (salary (Bitdiddle Ben) 60000) (lisp-value < 40000 60000))",
                    "(and (salary (Fect Cy D) 35000) (salary (Bitdiddle Ben) 60000) (lisp-value < 35000 60000))",
                    "(and (salary (Tweakit Lem E) 25000) (salary (Bitdiddle Ben) 60000) (lisp-value < 25000 60000))",
                    "(and (salary (Reasoner Louis) 30000) (salary (Bitdiddle Ben) 60000) (lisp-value < 30000 60000))",
                    "(and (salary (Cratchet Robert) 18000) (salary (Bitdiddle Ben) 60000) (lisp-value < 18000 60000))",
                    "(and (salary (Aull DeWitt) 25000) (salary (Bitdiddle Ben) 60000) (lisp-value < 25000 60000))",
                    "query: (and (supervisor ?person ?supervisor) (job ?supervisor ?job) (not (job ?supervisor (computer . ?type))))",
                    "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (job (Warbucks Oliver) (administration big wheel)) (not (job (Warbucks Oliver) (computer . ?type))))",
                    "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (job (Warbucks Oliver) (administration big wheel)) (not (job (Warbucks Oliver) (computer . ?type))))",
                    "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (job (Scrooge Eben) (accounting chief accountant)) (not (job (Scrooge Eben) (computer . ?type))))",
                    "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (job (Warbucks Oliver) (administration big wheel)) (not (job (Warbucks Oliver) (computer . ?type))))",
                )
        }
    })
