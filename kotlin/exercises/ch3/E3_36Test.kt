// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.36

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled

public class E3_36Test :
    FunSpec({
        test("Exercise 3.36: setValue consults the constraint, never the setter")
            .config(enabledOrReasonIf = { Enabled.disabled("pending solution") }) {
                val transcript = mutableListOf<String>()
                val a = Connector()
                a.connect(loggingConstraint(transcript, "logger"))
                a.setValue(10L, User)
                org.junit.jupiter.api.Assertions
                    .assertEquals(listOf("logger newValue"), transcript)
            }

        test("Exercise 3.36: forgetValue consults it again in list order")
            .config(enabledOrReasonIf = { Enabled.disabled("pending solution") }) {
                val transcript = mutableListOf<String>()
                val a = Connector()
                a.connect(loggingConstraint(transcript, "first"))
                a.connect(loggingConstraint(transcript, "second"))
                a.setValue(10L, User)
                a.forgetValue(User)
                org.junit.jupiter.api.Assertions.assertEquals(
                    listOf("second newValue", "first newValue", "second forgetValue", "first forgetValue"),
                    transcript,
                )
            }
    })
