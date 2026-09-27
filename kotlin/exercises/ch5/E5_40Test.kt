// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.40

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_40Test :
    FunSpec({
        test("Exercise 5.40: every variable reference reports its compile-time environment").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            compileTimeEnvDump() shouldBe emptyList()
        }
    })
