// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.19a

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E1_19aTest :
    FunSpec({
        test("Exercise 1.19a: the checked transform succeeds through Fib(91)").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(4_660_046_610_375_530_309L, ex_1_19a(91L))
        }
    })
