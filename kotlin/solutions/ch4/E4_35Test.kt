// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.35

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_35Test :
    FunSpec({
        test("Exercise 4.35: the Pythagorean triples between 1 and 20, then exhaustion") {
            triplesBetween20() shouldBe
                listOf("(3 4 5)", "(5 12 13)", "(6 8 10)", "(8 15 17)", "(9 12 15)", "(12 16 20)")
        }
    })
