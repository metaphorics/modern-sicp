// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.27

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_27Test :
    FunSpec({
        test("Exercise 1.27: every Carmichael number fools the Fermat test for every a < n").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(
                    mapOf(561L to true, 1105L to true, 1729L to true, 2465L to true, 2821L to true, 6601L to true),
                    ex_1_27(),
                )
        }
    })
