// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.14

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.structurallyEqual

public class E3_14Test :
    FunSpec({
        test("Exercise 3.14: mystery reverses the same cells in place").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val v = datumList(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d")) as PairCell
            val second = v.second as PairCell
            val third = second.second as PairCell
            val last = third.second as PairCell

            val w = mystery(v)

            org.junit.jupiter.api.Assertions
                .assertTrue(structurallyEqual(v, datumList(Symbol("a"))))
            org.junit.jupiter.api.Assertions.assertTrue(
                structurallyEqual(w, datumList(Symbol("d"), Symbol("c"), Symbol("b"), Symbol("a"))),
            )
            org.junit.jupiter.api.Assertions
                .assertTrue(w === last)
            org.junit.jupiter.api.Assertions
                .assertTrue(w.second === third)
        }
    })
