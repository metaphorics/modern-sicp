// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_12a

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_12aTest :
    FunSpec({
        test("Exercise 5.12a: the per-type census of the gcd machine") {
            gcdMachineCensus() shouldBe "(by type test 1 branch 1 assign 3 goto 1)"
        }
    })
