// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.14

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_14Test :
    FunSpec({
        test("the tree for 11 cents has 55 nodes") {
            ex_1_14() shouldBe 55L
        }
    })
