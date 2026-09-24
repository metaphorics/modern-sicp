// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.36

package sicp.ch3.exercises

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_36Test :
    FunSpec({
        test("setValue dispatches to the constraint once, the setter never") {
            val transcript = mutableListOf<String>()
            val a = Connector()
            a.connect(loggingConstraint(transcript, "logger"))
            a.setValue(10L, User) shouldBe Either.Right(Unit)
            transcript shouldBe listOf("logger newValue")
        }

        test("forgetValue dispatches again, in constraint-list order") {
            val transcript = mutableListOf<String>()
            val a = Connector()
            a.connect(loggingConstraint(transcript, "first"))
            a.connect(loggingConstraint(transcript, "second"))
            a.setValue(10L, User)
            a.forgetValue(User)
            val expected =
                listOf(
                    "second newValue",
                    "first newValue",
                    "second forgetValue",
                    "first forgetValue",
                )
            transcript shouldBe expected
        }

        test("the valueless neighbor b is never consulted at all") {
            val transcript = mutableListOf<String>()
            val a = Connector()
            val b = Connector()
            a.connect(loggingConstraint(transcript, "logger"))
            b.connect(loggingConstraint(transcript, "stray"))
            a.setValue(10L, User)
            transcript shouldBe listOf("logger newValue")
        }
    })
