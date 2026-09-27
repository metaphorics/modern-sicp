// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.27

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_27Test :
    FunSpec({
        test("Exercise 4.27: count 1, w 10, count 2, with the memoized re-display").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            lazyIdentityTranscript() shouldBe "1\n10\n2\n10\n2\n"
        }
    })
