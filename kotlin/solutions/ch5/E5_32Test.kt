// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.32

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_32Test :
    FunSpec({
        test("symbol calls and compound operators both evaluate correctly") {
            val lines = symbolOperatorRuns()
            lines
                .windowed(2)
                .filter { it[0] == ";;; EC-Eval value:" }
                .map { it[1] }
                .filter { it != "ok" } shouldBe listOf("36", "42")
            lines.any { it.contains("144") && it.contains("maximum-depth") } shouldBe true
        }
    })
