// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.43

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_43Test :
    FunSpec({
        test("Exercise 5.43: both body shapes are shown and the scanned program answers").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            scanOutShapes().first() shouldBe "the two programs answer alike: true"
        }
    })
