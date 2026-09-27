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
                    "query: (meeting ?division (Friday ?time))",
                    "(meeting administration (Friday 1pm))",
                    "query: (meeting-time (Hacker Alyssa P) (Wednesday ?time))",
                    "(meeting-time (Hacker Alyssa P) (Wednesday 4pm))",
                    "(meeting-time (Hacker Alyssa P) (Wednesday 3pm))",
                    "query: (meeting-time (Hacker Alyssa P) (Friday ?time))",
                )
        }
    })
