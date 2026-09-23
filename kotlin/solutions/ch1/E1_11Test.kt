// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.11

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_11Test :
    FunSpec({
        test("both processes compute f(10)") {
            ex_1_11(10L) shouldBe (1892L to 1892L)
        }
        test("the two processes agree on the base cases and the first steps past them") {
            (0L..12L).map { fRecursive(it) } shouldBe (0L..12L).map { fIterative(it) }
            fRecursive(5L) shouldBe 25L
        }
    })
