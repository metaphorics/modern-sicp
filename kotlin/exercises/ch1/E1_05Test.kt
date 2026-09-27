// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.5

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_05Test :
    FunSpec({
        test("Exercise 1.5: eager parameters evaluate, lambda parameters defer").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(1 to 0, ex_1_05())
        }
    })
