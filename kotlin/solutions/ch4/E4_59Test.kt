// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4_59

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_59Test :
    FunSpec({
        test("Exercise 4.59: the meeting queries") {
            meetingQueries() shouldBe
                listOf(
                    "?division = administration",
                    "?time = 1pm",
                    "?time = 4pm",
                    "?time = 3pm",
                )
        }
    })
