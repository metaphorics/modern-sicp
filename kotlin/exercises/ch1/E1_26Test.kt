// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.26

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_26Test :
    FunSpec({
        test("Exercise 1.26: Louis's version calls expmod far more often").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(8 to 191, ex_1_26())
        }
    })
