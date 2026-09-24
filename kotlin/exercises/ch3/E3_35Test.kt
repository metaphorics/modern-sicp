// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.35

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_35Test :
    FunSpec({
        test("Exercise 3.35: b infers a through the square root").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val a = Connector()
            val b = Connector()
            squarer(a, b)
            b.setValue(25L, User)
            org.junit.jupiter.api.Assertions
                .assertEquals(5L, a.value())
        }

        test("Exercise 3.35: a infers b through the square").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val a = Connector()
            val b = Connector()
            squarer(a, b)
            a.setValue(7L, User)
            org.junit.jupiter.api.Assertions
                .assertEquals(49L, b.value())
        }

        test("Exercise 3.35: negative b raises the book's error").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val a = Connector()
            val b = Connector()
            squarer(a, b)
            org.junit.jupiter.api.Assertions
                .assertThrows(
                    IllegalStateException::class.java,
                ) { b.setValue(-4L, User) }
        }
    })
