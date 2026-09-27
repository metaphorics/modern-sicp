// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.9

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_09Test :
    FunSpec({
        test("Exercise 1.9: both procedures add 4 and 5").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(9L to 9L, ex_1_09())
        }
    })
