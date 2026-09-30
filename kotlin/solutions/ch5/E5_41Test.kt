// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_41

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_41Test :
    FunSpec({
        test("Exercise 5.41: find-variable answers innermost-first addresses and null for unbound names") {
            val cenv = listOf(listOf("n"), listOf("product", "counter"))
            findVariable("counter", cenv) shouldBe LexicalAddress(1, 1)
            findVariable("n", cenv) shouldBe LexicalAddress(0, 0)
            findVariable("missing", cenv) shouldBe null
        }
    })
