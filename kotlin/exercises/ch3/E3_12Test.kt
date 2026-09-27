// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.12

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.vlist

public class E3_12Test :
    FunSpec({
        test("Exercise 3.12: append copies, appendBang splices").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val x = vlist(VSym("a"), VSym("b")) as VPair
            val y = vlist(VSym("c"), VSym("d")) as VPair
            org.junit.jupiter.api.Assertions
                .assertEquals("(a b c d)", append(x, y).toString())
            org.junit.jupiter.api.Assertions
                .assertEquals("(b)", x.cdr.toString())
            org.junit.jupiter.api.Assertions
                .assertEquals("(b c d)", appendBang(x, y).cdr.toString())
        }
    })
