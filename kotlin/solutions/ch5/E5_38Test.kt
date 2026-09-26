// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.38

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_38Test :
    FunSpec({
        test("open coding preserves arithmetic results and reduces factorial code") {
            val result = openCodedRuns()
            (
                result[1].substringAfter(": ").substringBefore(" ").toInt() <
                    result[0].substringAfter(": ").substringBefore(" ").toInt()
            ) shouldBe true
            result[2].endsWith("120") shouldBe true
            result[3] shouldBe "(+ 1 2 3 4): 10"
            result[4] shouldBe "(< 1 2): #t"
            result[5] shouldBe "(+ (* 2 3) (+ 4 5)): 15"
            result[6] shouldBe "(define (f) 40) (+ 1 2 (f)): 43"
            result[7] shouldBe "(+ (+ 1 2 3) 4): 10"
        }
    })
