// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.25

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E1_25Test :
    FunSpec({
        test("Alyssa's Long-wrapped shortcut disagrees with the correct answer") {
            ex_1_25() shouldBe Triple(3L, 0L, 3L)
        }
    })
