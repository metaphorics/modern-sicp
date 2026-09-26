// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.42

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_42Test :
    FunSpec({
        test("Exercise 5.42: the lexical accesses are shown and the applied example answers through them").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            lexicalAccessRuns() shouldBe emptyList()
        }
    })
