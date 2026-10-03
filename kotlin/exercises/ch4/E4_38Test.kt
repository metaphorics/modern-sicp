// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.38

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_38Test :
    FunSpec({
        test("Exercise 4.38: five solutions without the Smith-Fletcher clause").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            modifiedDwellingSolutions() shouldBe
                listOf(
                    "[[baker, 1], [cooper, 2], [fletcher, 4], [miller, 3], [smith, 5]]",
                    "[[baker, 1], [cooper, 2], [fletcher, 4], [miller, 5], [smith, 3]]",
                    "[[baker, 1], [cooper, 4], [fletcher, 2], [miller, 5], [smith, 3]]",
                    "[[baker, 3], [cooper, 2], [fletcher, 4], [miller, 5], [smith, 1]]",
                    "[[baker, 3], [cooper, 4], [fletcher, 2], [miller, 5], [smith, 1]]",
                )
        }

        test("Exercise 4.38: the brute force agrees with the amb search").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            modifiedDwellingBruteForce() shouldBe modifiedDwellingSolutions()
        }
    })
