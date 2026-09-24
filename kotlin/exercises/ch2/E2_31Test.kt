// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.31

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_31Test :
    FunSpec({
        test("Exercise 2.31: squareTreeViaTreeMap agrees with squareTree").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_31() shouldBe "(1 (4 (9 16) 25) (36 49))"
        }
    })
