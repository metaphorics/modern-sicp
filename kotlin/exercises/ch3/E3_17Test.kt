// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.17

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.VNil
import sicp.runtime.VSym
import sicp.runtime.cons
import sicp.runtime.vlist

public class E3_17Test :
    FunSpec({
        test("Exercise 3.17: distinct-pair count is 3 on every three-pair structure").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val shared = vlist(VSym("a"), VSym("b")) as sicp.runtime.VPair
            val says4 = cons(shared, shared.cdr)
            val q = cons(VSym("a"), VNil)
            val p = cons(q, q)
            val says7 = cons(p, p)
            org.junit.jupiter.api.Assertions
                .assertEquals(3, countDistinctPairs(vlist(VSym("a"), VSym("b"), VSym("c"))))
            org.junit.jupiter.api.Assertions
                .assertEquals(3, countDistinctPairs(says4))
            org.junit.jupiter.api.Assertions
                .assertEquals(3, countDistinctPairs(says7))
        }
    })
