// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.34

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_34Test :
    FunSpec({
        test("Exercise 3.34: setting b leaves a unknown").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val a = Connector()
            val b = Connector()
            louisSquarer(a, b)
            b.setValue(25L, User)
            org.junit.jupiter.api.Assertions
                .assertEquals(false, a.hasValue())
        }

        test("Exercise 3.34: the forward direction still squares").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val a = Connector()
            val b = Connector()
            louisSquarer(a, b)
            a.setValue(3L, User)
            org.junit.jupiter.api.Assertions
                .assertEquals(9L, b.value())
        }
    })
