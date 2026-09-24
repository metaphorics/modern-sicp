// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.33

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_33Test :
    FunSpec({
        test("Exercise 3.33: setting a and b gives c their average").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val a = Connector()
            val b = Connector()
            val c = Connector()
            averager(a, b, c)
            a.setValue(10L, User)
            b.setValue(20L, User)
            org.junit.jupiter.api.Assertions
                .assertEquals(15L, c.value())
        }

        test("Exercise 3.33: c can be given first and a inferred").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val a = Connector()
            val b = Connector()
            val c = Connector()
            averager(a, b, c)
            c.setValue(9L, User)
            a.setValue(8L, User)
            org.junit.jupiter.api.Assertions
                .assertEquals(10L, b.value())
        }
    })
