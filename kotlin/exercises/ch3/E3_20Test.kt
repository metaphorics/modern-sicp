// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.20

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.Whole

public class E3_20Test :
    FunSpec({
        test("Exercise 3.20: mutation through an alias is visible through x").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val x = proceduralPair(Whole(1L), Whole(2L))
            val alias = x
            alias.setFirst(Whole(17L))
            org.junit.jupiter.api.Assertions
                .assertEquals(Whole(17L), x.first())
            org.junit.jupiter.api.Assertions
                .assertEquals(Whole(2L), x.second())
        }
    })
