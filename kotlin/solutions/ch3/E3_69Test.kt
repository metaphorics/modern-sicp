// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.69

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_69Test :
    FunSpec({
        test("Exercise 3.69: the stream opens with the diagonal triple and its first row") {
            triples(integers, integers, integers).take(5) shouldBe
                listOf(
                    Triple(1L, 1L, 1L),
                    Triple(1L, 2L, 2L),
                    Triple(2L, 2L, 2L),
                    Triple(1L, 2L, 3L),
                    Triple(2L, 3L, 3L),
                )
        }

        test("Exercise 3.69: every triple in a 200-element prefix keeps i <= j <= k") {
            triples(integers, integers, integers).take(200).all { (i, j, k) -> i <= j && j <= k } shouldBe true
        }

        test("Exercise 3.69: the first Pythagorean triples, in stream order") {
            pythagoreanTriples().take(4) shouldBe
                listOf(
                    Triple(3L, 4L, 5L),
                    Triple(6L, 8L, 10L),
                    Triple(5L, 12L, 13L),
                    Triple(9L, 12L, 15L),
                )
        }
    })
