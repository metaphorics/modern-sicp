// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.37

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_37Test :
    FunSpec({
        test("Exercise 4.37: Ben's generator answers (3 4 5) first") {
            benFirstTriple() shouldBe "(3 4 5)"
        }

        test("Exercise 4.37: the book-order program backtracks 461 times to the first triple") {
            bookOrderBacktracksToFirst() shouldBe 461L
        }

        test("Exercise 4.37: Ben's program backtracks 42 times to the first triple") {
            benBacktracksToFirst() shouldBe 42L
        }
    })
