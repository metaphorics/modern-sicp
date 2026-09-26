// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.37

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_37Test :
    FunSpec({
        test("disabling preserving adds stack work without changing the answer") {
            val comparison = preservingComparison()
            val countPattern = Regex("(?:with preserving|without): (\\d+) statements, (\\d+) saves/restores")
            val enabledCounts =
                countPattern
                    .matchEntire(comparison[0])
                    ?.groupValues
                    ?.drop(1)
                    ?.map(String::toInt)
            val disabledCounts =
                countPattern
                    .matchEntire(comparison[1])
                    ?.groupValues
                    ?.drop(1)
                    ?.map(String::toInt)
            (enabledCounts != null && disabledCounts != null) shouldBe true
            val plainCounts = requireNotNull(enabledCounts)
            val blindCounts = requireNotNull(disabledCounts)
            (blindCounts[0] > plainCounts[0] && blindCounts[1] > plainCounts[1]) shouldBe true
            comparison.last() shouldBe "the answer stays 120 either way"
        }
    })
