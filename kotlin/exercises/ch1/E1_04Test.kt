// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.4

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_04Test :
    FunSpec({
        test("Exercise 1.4: the operator picked by an if").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(5L, ex_1_04(2L, -3L))
        }
    })
