// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.1.2

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** The name `size` denotes the value 2. */
public val size: Long = 2L

public class S1_1_2NamingTest :
    FunSpec({
        test("a name can be used wherever its value can appear") {
            size shouldBe 2L
            5L * size shouldBe 10L
        }
        test("compound results are named the same way") {
            val pi = 3.14159
            val radius = 10L
            pi * (radius * radius) shouldBe 314.159
            val circumference = 2L * pi * radius
            circumference shouldBe 62.8318
        }
    })
