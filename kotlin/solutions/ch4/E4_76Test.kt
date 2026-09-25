// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.76

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_76Test :
    FunSpec({
        test("Exercise 4.76: merge-and") {
            mergeAndDemos() shouldBe
                listOf(
                    "query: (merge-and (job ?x (computer programmer)) (supervisor ?x ?boss))",
                    "answers=2 same_answers_as_series: true compatibility checks: 18",
                    "query: (merge-and (supervisor ?x ?y) (job ?x ?job))",
                    "answers=8 same_answers_as_series: true compatibility checks: 80",
                )
        }
    })
