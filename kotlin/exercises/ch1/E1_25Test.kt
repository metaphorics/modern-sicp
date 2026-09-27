// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.25

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_25Test :
    FunSpec({
        test("Exercise 1.25: Alyssa's Long-wrapped shortcut disagrees with the correct answer").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(Triple(3L, 0L, 3L), ex_1_25())
        }
    })
