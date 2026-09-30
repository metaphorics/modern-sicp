// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_40

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_40Test :
    FunSpec({
        test("Exercise 5.40: the compile-time environment resolves each name to its address") {
            compileTimeEnvironmentDump() shouldBe
                listOf(
                    "product -> frame 1, offset 0",
                    "counter -> frame 1, offset 1",
                    "n -> frame 0, offset 0",
                    "missing -> unbound",
                )
        }
    })
