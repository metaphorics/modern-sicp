// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.20

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_20Test :
    FunSpec({
        test("remainder runs 4 times under eager evaluation") {
            ex_1_20() shouldBe 4
        }
    })
