// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.16

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.VNil
import sicp.runtime.VSym
import sicp.runtime.cons
import sicp.runtime.vlist

public class E3_16Test :
    FunSpec({
        test("Exercise 3.16: three-pair structures counted as 3, 4, and 7").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val shared = vlist(VSym("a"), VSym("b")) as sicp.runtime.VPair
            val says4 = cons(shared, shared.cdr)
            val q = cons(VSym("a"), VNil)
            val p = cons(q, q)
            val says7 = cons(p, p)
            org.junit.jupiter.api.Assertions
                .assertEquals(3, countPairs(vlist(VSym("a"), VSym("b"), VSym("c"))))
            org.junit.jupiter.api.Assertions
                .assertEquals(4, countPairs(says4))
            org.junit.jupiter.api.Assertions
                .assertEquals(7, countPairs(says7))
        }
    })
