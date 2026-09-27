// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.27

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_27Test :
    FunSpec({
        test("every Carmichael number fools the Fermat test for every a < n") {
            ex_1_27() shouldBe mapOf(561L to true, 1105L to true, 1729L to true, 2465L to true, 2821L to true, 6601L to true)
        }
        test("an ordinary composite does not fool the test for every a") {
            foolsFermat(100L) shouldBe false
        }
    })
