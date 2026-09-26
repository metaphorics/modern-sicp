// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.39

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_39Test :
    FunSpec({
        test("Exercise 5.39: the lexical machine runs the closure and cell program to one value").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            lexicalMachineRuns() shouldBe emptyList()
        }
    })
