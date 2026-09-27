// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.40

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_40Test :
    FunSpec({
        test("variable references carry the nested compile-time frames") {
            compileTimeEnvDump() shouldBe
                listOf(
                    "+ in (y z) (a b c d e) (x y)",
                    "x in (y z) (a b c d e) (x y)",
                    "y in (y z) (a b c d e) (x y)",
                    "z in (y z) (a b c d e) (x y)",
                )
        }
    })
