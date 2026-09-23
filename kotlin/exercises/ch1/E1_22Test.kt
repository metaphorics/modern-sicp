// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.22

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_22Test :
    FunSpec({
        test("Exercise 1.22: the three smallest primes above each threshold").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions.assertEquals(
                mapOf(
                    1_000L to listOf(1009L, 1013L, 1019L),
                    10_000L to listOf(10_007L, 10_009L, 10_037L),
                    100_000L to listOf(100_003L, 100_019L, 100_043L),
                    1_000_000L to listOf(1_000_003L, 1_000_033L, 1_000_037L),
                ),
                ex_1_22(),
            )
        }
    })
