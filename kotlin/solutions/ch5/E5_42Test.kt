// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_42

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_42Test :
    FunSpec({
        test("Exercise 5.42: the addressing model resolves names to distinct working addresses") {
            lexicalAddressingReport() shouldBe
                listOf(
                    "every name resolves to an address: true",
                    "the addresses are distinct: true",
                    "the lookups answer the bound values: true",
                )
        }
    })
