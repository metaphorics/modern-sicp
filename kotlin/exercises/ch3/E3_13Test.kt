// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.13

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList

public class E3_13Test :
    FunSpec({
        test("Exercise 3.13: makeCycle closes z onto itself").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val z = makeCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c")) as PairCell)
            val second = z.second as PairCell
            val third = second.second as PairCell
            org.junit.jupiter.api.Assertions
                .assertTrue(third.second === z)
            org.junit.jupiter.api.Assertions
                .assertFalse(second === z)
            org.junit.jupiter.api.Assertions
                .assertFalse(third === z)

            var cursor: Datum = z
            repeat(9) {
                cursor = (cursor as PairCell).second
                org.junit.jupiter.api.Assertions
                    .assertFalse(cursor === Empty)
            }
            org.junit.jupiter.api.Assertions
                .assertTrue(cursor === z)
        }
    })
