// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.28

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_28Test :
    FunSpec({
        test("Exercise 1.28: Miller-Rabin is not fooled by any Carmichael number").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(
                    mapOf(561L to false, 1105L to false, 1729L to false, 2465L to false, 2821L to false, 6601L to false),
                    ex_1_28(),
                )
        }
    })
