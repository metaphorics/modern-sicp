// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.18

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.VSym
import sicp.runtime.vlist

public class E3_18Test :
    FunSpec({
        test("Exercise 3.18: a made cycle is found, a plain chain is not").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val z = sicp.ch3.exercises.makeCycle(vlist(VSym("a"), VSym("b"), VSym("c")) as sicp.runtime.VPair)
            org.junit.jupiter.api.Assertions
                .assertEquals(true, containsCycle(z))
            org.junit.jupiter.api.Assertions
                .assertEquals(false, containsCycle(vlist(VSym("a"), VSym("b"), VSym("c"))))
        }
    })
