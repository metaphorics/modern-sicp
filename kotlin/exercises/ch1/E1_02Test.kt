// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.2

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_02Test :
    FunSpec({
        test("Exercise 1.2: the fraction as pure nested calls").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(-0.24666666666666667, ex_1_02(), 0.0)
        }
    })
