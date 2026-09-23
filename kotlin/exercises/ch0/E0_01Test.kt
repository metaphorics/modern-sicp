// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E0_01Test :
    FunSpec({
        test("Exercise 0.1: three Scheme sessions in Kotlin").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(
                    listOf(486L, 100L, 12L, 1L, 6L, 19L, 4L, 16L, 6L, 16L, 441L, 49L, 81L),
                    ex_0_01(),
                )
        }
    })
