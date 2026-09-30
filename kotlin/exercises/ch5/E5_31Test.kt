// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.31

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_31Test :
    FunSpec({
        test("Exercise 5.31: the four combinations report their surviving save/restore pairs").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val report = superfluousSaves()
            report.size shouldBe 6
            report[4] shouldBe "every save pairs with one restore: true"
            report[5] shouldBe "compiled and direct runs agree: true"
        }
    })
