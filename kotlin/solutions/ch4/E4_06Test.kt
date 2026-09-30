// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.6: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_06Test :
    FunSpec({
        test("Exercise 4.6: ordinary and derived let answer alike") {
            letEquivalenceTranscript() shouldBe "7\n7\n"
        }

        test("Exercise 4.6: the derived binding evaluates its body") {
            letBodyTranscript() shouldBe "7\n"
        }

        test("Exercise 4.6: nested bindings evaluate in their frames") {
            letNestedTranscript() shouldBe "3\n"
        }
    })
