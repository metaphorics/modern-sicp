// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.12

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.structurallyEqual

public class E3_12Test :
    FunSpec({
        test("Exercise 3.12: append copies, appendBang splices").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val x = datumList(Symbol("a"), Symbol("b")) as PairCell
            val y = datumList(Symbol("c"), Symbol("d")) as PairCell
            val originalTail = x.second as PairCell

            org.junit.jupiter.api.Assertions.assertTrue(
                structurallyEqual(append(x, y), datumList(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d"))),
            )
            org.junit.jupiter.api.Assertions
                .assertTrue(x.second === originalTail)

            val result = appendBang(x, y)
            org.junit.jupiter.api.Assertions
                .assertTrue(result === x)
            org.junit.jupiter.api.Assertions.assertTrue(
                structurallyEqual(x.second, datumList(Symbol("b"), Symbol("c"), Symbol("d"))),
            )
            org.junit.jupiter.api.Assertions
                .assertTrue(originalTail.second === y)
        }
    })
