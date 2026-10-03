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

        test("Exercise 4.37: computing the third component prunes failed requirements") {
            val book = bookOrderBacktracksToFirst()
            val ben = benBacktracksToFirst()
            (book > 0L) shouldBe true
            (ben > 0L) shouldBe true
            (ben < book) shouldBe true
        }
    })
