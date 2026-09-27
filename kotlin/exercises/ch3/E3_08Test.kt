// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.8

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_08Test :
    FunSpec({
        test("Exercise 3.8: f(0) + f(1) is 0, guaranteed by Kotlin's left-to-right order").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val f = makeF()
            org.junit.jupiter.api.Assertions
                .assertEquals(0, f(0) + f(1))
        }
    })
