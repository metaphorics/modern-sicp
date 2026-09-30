// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_43

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_43Test :
    FunSpec({
        test("Exercise 5.43: scanning out the internal definitions preserves the answers") {
            scanOutInternalDefines().first() shouldBe "the two programs answer alike: true"
        }
    })
