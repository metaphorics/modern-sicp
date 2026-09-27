// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E0_05Test :
    FunSpec({
        test("Exercise 0.5: the Sequence recomputes, the LStream does not").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(Pair(10, 6), ex_0_05())
        }
    })
