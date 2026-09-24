// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.47

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_47Test :
    FunSpec({
        test("Exercise 2.47: both frame constructors agree on the same frame").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_47() shouldBe true
        }
    })
