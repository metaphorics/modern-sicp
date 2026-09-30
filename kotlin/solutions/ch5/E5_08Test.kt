// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_08

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_08Test :
    FunSpec({
        test("Exercise 5.8: a doubly defined label is refused at assembly time") {
            duplicateLabelOutcome() shouldBe "DuplicateLabel: here"
        }
    })
