// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.44

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_44Test :
    FunSpec({
        test("Exercise 5.44: shadowed parameters open-code nothing, free names open-code all").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            openCodeShadowingCounts() shouldBe emptyList()
        }
    })
