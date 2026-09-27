// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.19

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.VSym
import sicp.runtime.vlist

public class E3_19Test :
    FunSpec({
        test("Exercise 3.19: constant-space detection agrees with 3.18").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val z = sicp.ch3.exercises.makeCycle(vlist(VSym("a"), VSym("b"), VSym("c")) as sicp.runtime.VPair)
            org.junit.jupiter.api.Assertions
                .assertEquals(true, containsCycleConstantSpace(z))
            org.junit.jupiter.api.Assertions
                .assertEquals(false, containsCycleConstantSpace(vlist(VSym("a"), VSym("b"), VSym("c"))))
        }
    })
