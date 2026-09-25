// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.29

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_29Test :
    FunSpec({
        test("Exercise 4.29: memoized thunks compute once per argument").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            memoizedCountsTranscript() shouldBe "100\n1\n1000\n2\n"
        }

        test("Exercise 4.29: unmemoized thunks recompute at every demand").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            unmemoizedCountsTranscript() shouldBe "100\n2\n1000\n5\n"
        }
    })
