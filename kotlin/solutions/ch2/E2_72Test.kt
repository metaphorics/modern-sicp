// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.72

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_72Test :
    FunSpec({
        test("n = 10: the most frequent symbol costs n - 1 = 9 steps, the least frequent n(n-1)/2 = 45") {
            ex_2_72(10) shouldBe (9 to 45)
        }
        test("n = 5: the most frequent symbol costs 4 steps, the least frequent 10") {
            ex_2_72(5) shouldBe (4 to 10)
        }
        test("most-frequent-symbol cost grows linearly: doubling n roughly doubles the steps") {
            val (steps10, _) = ex_2_72(10)
            val (steps20, _) = ex_2_72(20)
            steps20 shouldBe 2 * steps10 + 1
        }
        test("least-frequent-symbol cost grows quadratically: f(2n) = 4 f(n) + n for f(n) = n(n-1)/2") {
            val (_, steps10) = ex_2_72(10)
            val (_, steps20) = ex_2_72(20)
            steps20 shouldBe 4 * steps10 + 10
        }
    })
