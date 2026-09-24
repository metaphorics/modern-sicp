// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.69

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_69Test :
    FunSpec({
        test(
            "Exercise 2.69: the generated Huffman tree for the A-through-H alphabet carries the total weight 17",
        ).config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_69().weight shouldBe 17L
        }
    })
