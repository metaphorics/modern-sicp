// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.3: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_03Test :
    FunSpec({
        test("Exercise 4.3: the table answers installed tags and misses others until installed") {
            tableDispatchTranscript() shouldBe "42\nerror\n5\n"
        }
    })
