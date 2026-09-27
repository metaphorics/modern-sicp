// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.50

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E3_50Test :
    FunSpec({
        test("Exercise 3.50: the n-stream map applies the procedure in lockstep").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val sums =
                streamMapN(
                    { triple -> triple.sum() },
                    listOf(
                        (1L..5L).asSequence(),
                        (10L..50L step 10L).asSequence(),
                        (100L..500L step 100L).asSequence(),
                    ),
                )
            sums.toList() shouldBe listOf(111L, 222L, 333L, 444L, 555L)
        }

        test("Exercise 3.50: the result runs out at the shortest stream").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val products =
                streamMapN(
                    { pair -> pair[0] * pair[1] },
                    listOf((1L..10L).asSequence(), (2L..4L).asSequence()),
                )
            products.toList() shouldBe listOf(2L, 6L, 12L)
        }

        test("Exercise 3.50: an empty stream empties the result").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val none =
                streamMapN(
                    { triple -> triple.sum() },
                    listOf((1L..3L).asSequence(), emptySequence<Long>(), (7L..9L).asSequence()),
                )
            none.toList() shouldBe emptyList()
        }
    })
