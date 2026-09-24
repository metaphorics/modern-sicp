// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.42

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_42Test :
    FunSpec({
        test("queens(8) finds all 92 solutions") {
            ex_2_42() shouldBe 92
        }
        test("queens(6) finds the 4 known solutions") {
            queens(6).size shouldBe 4
        }
        test("every reported board is safe in every column") {
            for (board in queens(7)) {
                board.forEachIndexed { index, _ -> safeCol(index + 1, board) shouldBe true }
            }
        }
    })
