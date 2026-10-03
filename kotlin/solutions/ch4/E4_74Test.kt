// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_74

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_74Test :
    FunSpec({
        test("Exercise 4.74: the simple flatmap") {
            simpleFlatmapDemo() shouldBe listOf("?y = [Minnie, Mouse]")
        }
    })
