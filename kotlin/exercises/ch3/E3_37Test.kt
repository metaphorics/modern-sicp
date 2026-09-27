// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.37

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_37Test :
    FunSpec({
        test("Exercise 3.37: the book's session, expression style").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val c = Connector()
            val f = celsiusFahrenheit(c)
            c.setValue(25L, User)
            org.junit.jupiter.api.Assertions
                .assertEquals(77L, f.value())
            c.forgetValue(User)
            f.setValue(212L, User)
            org.junit.jupiter.api.Assertions
                .assertEquals(100L, c.value())
        }

        test("Exercise 3.37: the combinators compose and infer both ways").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val x = cConst(10L)
            val y = cConst(3L)
            org.junit.jupiter.api.Assertions
                .assertEquals(13L, cPlus(x, y).value())
            org.junit.jupiter.api.Assertions
                .assertEquals(7L, cMinus(x, y).value())
            org.junit.jupiter.api.Assertions
                .assertEquals(30L, cMul(x, y).value())
            org.junit.jupiter.api.Assertions
                .assertEquals(10L, cDiv(cMul(x, y), y).value())
        }
    })
