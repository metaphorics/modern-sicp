// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_79

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_79Test :
    FunSpec({
        test("Exercise 4.79: scoped versus renaming") {
            scopedVersusRenaming() shouldBe listOf("Hacker", "programmer", "Fect", "programmer")
        }
    })
