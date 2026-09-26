// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.36

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_36Test :
    FunSpec({
        test("compiled operands run right-to-left by default and can run left-to-right") {
            val result = operandOrderRuns()
            result[0] shouldBe "default order: 2 1"
            result[1] shouldBe "left-to-right order: 1 2"
            val counts =
                Regex("instruction counts: (\\d+) = (\\d+): true")
                    .matchEntire(result[2])
                    ?.groupValues
                    ?.drop(1)
            val parsedCounts = requireNotNull(counts)
            parsedCounts[0] shouldBe parsedCounts[1]
        }
    })
