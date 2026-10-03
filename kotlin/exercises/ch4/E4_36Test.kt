// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.36

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_36Test :
    FunSpec({
        test("Exercise 4.36: the fair generator's first six unbounded triples").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            fairTriplesFirstSix() shouldBe
                listOf("[3, 4, 5]", "[6, 8, 10]", "[5, 12, 13]", "[9, 12, 15]", "[8, 15, 17]", "[12, 16, 20]")
        }

        test("Exercise 4.36: the naive replacement never leaves its first choices").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            naiveBudgetFault() shouldBe "BudgetExhausted after 600 choices"
        }
    })
