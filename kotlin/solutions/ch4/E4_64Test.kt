// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_64

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_64Test :
    FunSpec({
        test("Exercise 4.64: Louis's swapped outranked-by") {
            louisOutranked() shouldBe
                listOf(
                    "louis anchored: first answer arrives (1 frame)",
                    "book order: completes with 1 frame",
                    "loop detector bounds louis: stream completes",
                )
        }
    })
