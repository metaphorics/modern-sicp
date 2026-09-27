// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.41

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_41Test :
    FunSpec({
        test("Exercise 5.41: the three book cases over the three-frame environment are pinned").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            findVariableLookups() shouldBe emptyList()
        }
    })
