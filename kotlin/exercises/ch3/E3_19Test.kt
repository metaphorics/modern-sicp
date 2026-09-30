// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.19

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.pair

public class E3_19Test :
    FunSpec({
        test("Exercise 3.19: constant-space detection catches loops and rejects chains").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val cycle = makeCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c")) as PairCell)
            val selfLoop = pair(Symbol("self"), Empty)
            selfLoop.second = selfLoop

            org.junit.jupiter.api.Assertions
                .assertTrue(containsCycleConstantSpace(cycle))
            org.junit.jupiter.api.Assertions
                .assertTrue(containsCycleConstantSpace(selfLoop))
            org.junit.jupiter.api.Assertions
                .assertFalse(containsCycleConstantSpace(datumList(Symbol("a"), Symbol("b"))))
            org.junit.jupiter.api.Assertions.assertFalse(
                containsCycleConstantSpace(datumList(Symbol("a"), Symbol("b"), Symbol("c"))),
            )
        }
    })
