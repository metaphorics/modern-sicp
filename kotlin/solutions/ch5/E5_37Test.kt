// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_37

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_37Test :
    FunSpec({
        test("Exercise 5.37: the waste analysis pairs every save with its restore") {
            preservingWaste().last() shouldBe "every save pairs with one restore: true"
        }
    })
