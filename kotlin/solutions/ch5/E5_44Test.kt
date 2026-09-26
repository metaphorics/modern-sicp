// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.44

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_44Test :
    FunSpec({
        test("open coding excludes lexical shadows and warns on a redefinition") {
            openCodeShadowingCounts() shouldBe
                listOf(
                    "shadowed parameters: 0 open-coded operations",
                    "free names: 3 open-coded operations",
                    "warnings: open-coded primitive + is rebound",
                    "open-coded names: + - * < =",
                )
        }
    })
