// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.24

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_24Test :
    FunSpec({
        test("the clause loop answers the same values the derived form did") {
            condBasicFormRuns().filterNot { it.startsWith(";;;") } shouldBe
                listOf("ok", "zero", "one", "many", "#t", "#f")
        }

        test("the selected clause's recursive call stays in tail position: depth constant in n") {
            condTailPositionDepths(listOf(10, 50, 100, 200)) shouldBe List(4) { 10 }
        }
    })
