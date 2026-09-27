// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.43

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_43Test :
    FunSpec({
        test("queensSlow(6) finds the same solution set as queens(6)") {
            queensSlow(6).toSet() shouldBe queens(6).toSet()
        }
        test("ex_2_43 reports the 4 solutions") {
            ex_2_43() shouldBe 4
        }
    })
