// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.20

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_20Test :
    FunSpec({
        test("Exercise 1.20: remainder runs 4 times under eager evaluation").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(4, ex_1_20())
        }
    })
