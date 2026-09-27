// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.71

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.runtime.take

public class E3_71Test :
    FunSpec({
        test("Exercise 3.71: the first six Ramanujan numbers").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ramanujanNumbers().take(6) shouldBe
                listOf(1729L, 4104L, 13832L, 20683L, 32832L, 39312L)
        }

        test("Exercise 3.71: the numbers arrive in increasing order").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val six = ramanujanNumbers().take(6)
            six.zipWithNext().all { (a, b) -> a < b } shouldBe true
        }
    })
