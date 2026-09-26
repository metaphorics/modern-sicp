// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.49

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_49Test :
    FunSpec({
        test("Exercise 5.49: each form's prompt, run, and printed value is grouped").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            readCompileExecutePrint() shouldBe emptyList()
        }
    })
