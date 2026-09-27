// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.13

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.vlist

public class E3_13Test :
    FunSpec({
        test("Exercise 3.13: makeCycle closes z onto itself").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val z = makeCycle(vlist(VSym("a"), VSym("b"), VSym("c")) as VPair)
            val second = z.cdr as VPair
            val third = second.cdr as VPair
            org.junit.jupiter.api.Assertions
                .assertTrue(third.cdr === z)
        }
    })
