// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.48

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_48Test :
    FunSpec({
        test("compile-and-run compiles once and the block answers the session") {
            compileAndRunSession() shouldBe
                listOf(
                    "compile-and-run answers: ok",
                    "compiled block answers: ok 120",
                )
        }
    })
