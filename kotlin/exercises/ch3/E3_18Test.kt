// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.18

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList

public class E3_18Test :
    FunSpec({
        test("Exercise 3.18: a made cycle is found, a plain chain is not").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val cycle = makeCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c")) as PairCell)
            org.junit.jupiter.api.Assertions
                .assertTrue(containsCycle(cycle))
            org.junit.jupiter.api.Assertions
                .assertFalse(containsCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c"))))
        }
    })
