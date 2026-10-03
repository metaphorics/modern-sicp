// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.16

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.pair

public class E3_16Test :
    FunSpec({
        test("Exercise 3.16: three-pair structures counted as 3, 4, and 7").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val shared = datumList(Symbol("a"), Symbol("b")) as PairCell
            val says4 = pair(shared, shared.second)
            val q = pair(Symbol("a"), Empty)
            val p = pair(q, q)
            val says7 = pair(p, p)
            org.junit.jupiter.api.Assertions
                .assertEquals(3, countPairs(datumList(Symbol("a"), Symbol("b"), Symbol("c"))))
            org.junit.jupiter.api.Assertions
                .assertEquals(4, countPairs(says4))
            org.junit.jupiter.api.Assertions
                .assertEquals(7, countPairs(says7))
        }
    })
