// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.15

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.cons
import sicp.runtime.vlist

public class E3_15Test :
    FunSpec({
        test("Exercise 3.15: set-to-wow on the shared z1 versus the unshared z2").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val z1 = cons(x, x)
            val z2 = cons(vlist(VSym("a"), VSym("b")), vlist(VSym("a"), VSym("b")))
            org.junit.jupiter.api.Assertions
                .assertEquals("((wow b) wow b)", setToWow(z1).toString())
            org.junit.jupiter.api.Assertions
                .assertEquals("((wow b) a b)", setToWow(z2).toString())
        }
    })
