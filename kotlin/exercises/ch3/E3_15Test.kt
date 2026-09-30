// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.15

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.structurallyEqual

public class E3_15Test :
    FunSpec({
        test("Exercise 3.15: mutation follows one shared pair but not an equal copy").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val shared = datumList(Symbol("a"), Symbol("b")) as PairCell
            val z1 = pair(shared, shared)
            val left = datumList(Symbol("a"), Symbol("b"))
            val right = datumList(Symbol("a"), Symbol("b"))
            val z2 = pair(left, right)

            org.junit.jupiter.api.Assertions
                .assertTrue(z1.first === z1.second)
            org.junit.jupiter.api.Assertions
                .assertFalse(z2.first === z2.second)
            org.junit.jupiter.api.Assertions.assertTrue(
                structurallyEqual(setToWow(z1), pair(datumList(Symbol("wow"), Symbol("b")), datumList(Symbol("wow"), Symbol("b")))),
            )
            org.junit.jupiter.api.Assertions.assertTrue(
                structurallyEqual(setToWow(z2), pair(datumList(Symbol("wow"), Symbol("b")), datumList(Symbol("a"), Symbol("b")))),
            )
        }
    })
