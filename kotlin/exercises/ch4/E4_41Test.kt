// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.41

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_41Test :
    FunSpec({
        test("Exercise 4.41: the ordinary solver answers the puzzle").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            kotlinSolverAnswer() shouldBe "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
        }

        test("Exercise 4.41: the plain solver tests the whole floor grid").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            kotlinSolverTests() shouldBe 1471
        }
    })
