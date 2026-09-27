// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 83

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_83Test :
    FunSpec({
        test("each level raises one rung") {
            ex_2_83() shouldBe listOf("7/1", "0.75", "2.5+0.0i")
        }

        test("the top of the tower has no raise install") {
            val table = NumTable()
            installRaise(table)
            val result = arrow.core.raise.either { raiseOf(table, Complex(Rect(1.0, 1.0))) }
            result.leftOrNull() shouldBe GenError.NoMethod("raise", listOf("complex"))
        }
    })
