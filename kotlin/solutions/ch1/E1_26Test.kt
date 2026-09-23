// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.26

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_26Test :
    FunSpec({
        test("Louis's version calls expmod far more often") {
            ex_1_26() shouldBe (8 to 191)
        }
    })
