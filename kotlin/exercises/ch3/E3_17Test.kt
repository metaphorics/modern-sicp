// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.17

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.pair

public class E3_17Test :
    FunSpec({
        test("Exercise 3.17: distinct-pair count is 3 on every three-pair structure").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val shared = datumList(Symbol("a"), Symbol("b")) as PairCell
            val says4 = pair(shared, shared.second)
            val q = pair(Symbol("a"), Empty)
            val p = pair(q, q)
            val says7 = pair(p, p)
            org.junit.jupiter.api.Assertions
                .assertEquals(3, countDistinctPairs(datumList(Symbol("a"), Symbol("b"), Symbol("c"))))
            org.junit.jupiter.api.Assertions
                .assertEquals(3, countDistinctPairs(says4))
            org.junit.jupiter.api.Assertions
                .assertEquals(3, countDistinctPairs(says7))
        }
    })
