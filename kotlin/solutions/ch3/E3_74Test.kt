// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.74

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_74Test :
    FunSpec({
        test("Exercise 3.74: the detector reads sign changes, counting 0 as positive") {
            signChangeDetector(2.0, 1.0) shouldBe 0L
            signChangeDetector(-0.1, 0.5) shouldBe -1L
            signChangeDetector(0.2, -0.5) shouldBe 1L
            signChangeDetector(0.0, -1.0) shouldBe 1L
            signChangeDetector(-1.0, 0.0) shouldBe -1L
            signChangeDetector(0.0, 1.0) shouldBe 0L
            signChangeDetector(-0.5, 0.0) shouldBe -1L
        }

        test("Exercise 3.74: the recursive extractor reproduces the book's crossing row") {
            makeZeroCrossings(senseData, 0.0).take(20) shouldBe
                listOf(0L, 0L, 0L, 0L, 0L, -1L, 0L, 0L, 0L, 0L, 1L, 0L, 0L)
        }

        test("Exercise 3.74: Eva's zipStream construction gives the same crossings") {
            zeroCrossings.take(20) shouldBe
                listOf(0L, 0L, 0L, 0L, 0L, -1L, 0L, 0L, 0L, 0L, 1L, 0L, 0L)
        }
    })
