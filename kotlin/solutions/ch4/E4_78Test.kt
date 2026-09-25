// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_78

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldContain

public class E4_78Test :
    FunSpec({
        test("Exercise 4.78: the amb port") {
            val demos = ambQueryDemos()
            demos.size shouldBe 7
            demos.joinToString("\n") shouldContain ";;; Amb-Eval value:"
        }
    })
