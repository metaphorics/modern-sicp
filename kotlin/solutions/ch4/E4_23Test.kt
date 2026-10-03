// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.23: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_23Test :
    FunSpec({
        test("Exercise 4.23: the text analyzer defers every effect to the run") {
            textSequenceTranscript() shouldBe "0\n30\n"
        }

        test("Exercise 4.23: Alyssa's analyzer runs the forms during analysis") {
            alyssaSequenceTranscript() shouldBe "30\n60\n"
        }
    })
