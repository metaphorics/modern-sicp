// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_73

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_73Test :
    FunSpec({
        test("Exercise 4.73: the flatten-stream delay") {
            flattenDelayDebate() shouldBe
                listOf(
                    "delayed_first3=3 source_calls=4",
                    "eager_diverged_before_first_answer=true source_calls=2001",
                    "finite_orders_agree=true",
                )
        }
    })
