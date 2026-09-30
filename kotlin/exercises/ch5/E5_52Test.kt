// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.52

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_52Test :
    FunSpec({
        test("Exercise 5.52: the C backend builds a C interpreter that runs its object program").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            compilerToCRuns() shouldBe
                listOf(
                    "the emitter produced a C function: true",
                    "the artifact carries the compilation's labels and registers: true",
                    "the reference artifact is distinct text: true",
                )
        }
    })
