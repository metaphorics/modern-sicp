// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_68

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_68Test :
    FunSpec({
        test("Exercise 4.68: the reverse queries") {
            reverseQueries() shouldBe
                listOf(
                    "?x = [3, 2, 1]",
                    "?x = [d, c, b, a]",
                    "?x = [3, 2, 1]",
                )
        }
    })
