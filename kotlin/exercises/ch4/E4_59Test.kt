// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.59

package sicp.ch4.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E4_59Test :
    FunSpec({
        test("Exercise 4.59: the meeting queries").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            meetingQueries() shouldBe
                listOf(
                    "?division = administration",
                    "?time = 1pm",
                    "?time = 4pm",
                    "?time = 3pm",
                )
        }
    })
