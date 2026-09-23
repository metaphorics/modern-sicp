// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_01Test :
    FunSpec({
        test("Exercise 1.1: evaluate a sequence of expressions in order").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions.assertEquals(listOf(10L, 20L), ex_1_01())
        }
    })
