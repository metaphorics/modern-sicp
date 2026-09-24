// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.46

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_46Test :
    FunSpec({
        test("Exercise 2.46: addVect(makeVect(1.0, 2.0), makeVect(3.0, 4.5)) is (4.0, 6.5)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_46() shouldBe Vect(4.0, 6.5)
        }
    })
