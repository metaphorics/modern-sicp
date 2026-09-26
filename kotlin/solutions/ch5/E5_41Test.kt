// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.41

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_41Test :
    FunSpec({
        test("find-variable returns frame and displacement or not-found") {
            findVariableLookups() shouldBe listOf("c: (1 2)", "x: (2 0)", "w: not-found")
        }
    })
