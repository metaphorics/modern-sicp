// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.34

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_34Test :
    FunSpec({
        test("iterative factorial keeps stack depth constant as its input grows") {
            val result = iterativeFactorialCompilation()
            result[0].substringAfter(": ").toInt() shouldBe 2
            val depths = result.slice(2..4).map { it.substringAfter(": ").toInt() }
            depths.distinct().size shouldBe 1
            result.last() shouldBe "the depths are equal: true"
        }
    })
