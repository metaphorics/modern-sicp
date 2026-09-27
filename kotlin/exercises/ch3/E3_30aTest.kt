// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.30a

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_30aTest :
    FunSpec({
        test("Exercise 3.30a: the assembled ripple-carry result equals a + b + cIn for the boundary cases").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(0, rippleAdd(0, 0, 0))
            org.junit.jupiter.api.Assertions
                .assertEquals(30, rippleAdd(15, 15, 0))
        }
    })
