// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.51

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E3_51Test :
    FunSpec({
        test("Exercise 3.51: the memoized map shows each element once").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val shown = shownInterval(0, 10, memoized = true)
            shown.log shouldBe listOf("0")
            streamRef(shown.stream, 5)
            shown.log shouldBe listOf("0", "1", "2", "3", "4", "5")
            streamRef(shown.stream, 7)
            shown.log shouldBe listOf("0", "1", "2", "3", "4", "5", "6", "7")
            streamRef(shown.stream, 7) shouldBe 7L
        }

        test("Exercise 3.51: the unmemoized twin re-records the cells a fresh walk passes").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val shown = shownInterval(0, 10, memoized = false)
            streamRef(shown.stream, 5)
            shown.log shouldBe listOf("0", "1", "2", "3", "4", "5")

            val rerun = shownInterval(0, 10, memoized = false)
            streamRef(rerun.stream, 7) shouldBe 7L
            rerun.log shouldBe listOf("0", "1", "2", "3", "4", "5", "6", "7")
            rerun.log.subList(1, 6) shouldBe listOf("1", "2", "3", "4", "5")
        }
    })
