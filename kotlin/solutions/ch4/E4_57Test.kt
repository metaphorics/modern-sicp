// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_57

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_57Test :
    FunSpec({
        test("Exercise 4.57: the can-replace queries") {
            canReplaceQueries() shouldBe
                listOf(
                    "query: (can-replace ?x (Fect Cy D))",
                    "(can-replace (Bitdiddle Ben) (Fect Cy D))",
                    "(can-replace (Hacker Alyssa P) (Fect Cy D))",
                    "query: (and (can-replace ?person-1 ?person-2) (salary ?person-1 ?salary-1) (salary ?person-2 ?salary-2) (lisp-value < ?salary-1 ?salary-2))",
                    "(and (can-replace (Fect Cy D) (Hacker Alyssa P)) (salary (Fect Cy D) 35000) (salary (Hacker Alyssa P) 40000) (lisp-value < 35000 40000))",
                    "(and (can-replace (Aull DeWitt) (Warbucks Oliver)) (salary (Aull DeWitt) 25000) (salary (Warbucks Oliver) 150000) (lisp-value < 25000 150000))",
                )
        }
    })
