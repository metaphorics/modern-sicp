// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.39

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_39Test :
    FunSpec({
        test("Exercise 4.39: both orders answer the same assignment").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            dwellingAnswer() shouldBe "[[baker, 3], [cooper, 2], [fletcher, 4], [miller, 5], [smith, 1]]"
        }

        test("Exercise 4.39: the book order costs 1470 backtracks to the first answer").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            bookOrderBacktracks() shouldBe 1470L
        }

        test("Exercise 4.39: the Fletcher-first reorder costs the same 1470").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            reorderedBacktracks() shouldBe 1470L
        }
    })
