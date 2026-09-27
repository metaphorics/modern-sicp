// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.14

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.vlist

public class E3_14Test :
    FunSpec({
        test("Exercise 3.14: mystery reverses v in place").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val v = vlist(VSym("a"), VSym("b"), VSym("c"), VSym("d")) as VPair
            val w = mystery(v)
            org.junit.jupiter.api.Assertions
                .assertEquals("(a)", v.toString())
            org.junit.jupiter.api.Assertions
                .assertEquals("(d c b a)", w.toString())
        }
    })
