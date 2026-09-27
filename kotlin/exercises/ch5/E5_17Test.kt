// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.17

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_17Test :
    FunSpec({
        test("Exercise 5.17: traced lines with their labels").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            labelTracedGcdTrace() shouldBe emptyList()
        }
    })
