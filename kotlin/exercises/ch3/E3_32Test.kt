// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.32

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_32Test :
    FunSpec({
        test("Exercise 3.32: two actions added to one segment run in insertion order").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            org.junit.jupiter.api.Assertions
                .assertEquals(listOf("first", "second"), sameSegmentRunOrder())
        }
    })
