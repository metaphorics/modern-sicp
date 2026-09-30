// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.6: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_06Test :
    FunSpec({
        test("Exercise 4.6: single derivation, simultaneous shadowing, nested agreement") {
            letDerivedTranscript() shouldBe "6\n5\n7\n"
        }
    })
