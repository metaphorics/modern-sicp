// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.4

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_04Test :
    FunSpec({
        test("Exercise 2.4: cdrFn(consFn(3, 4)) is 4").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_04() shouldBe 4L
        }
    })
