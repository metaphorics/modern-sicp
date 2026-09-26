// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.42

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_42Test :
    FunSpec({
        test("lexical addressing compiles addresses and runs to 180") {
            lexicalAddressRuns() shouldBe
                listOf(
                    "(assign val (op lexical-address-lookup) (const (0 1)) (reg env))",
                    "(assign val (op lexical-address-lookup) (const (0 0)) (reg env))",
                    "(assign val (op lexical-address-lookup) (const (2 0)) (reg env))",
                    "lexical run: 180",
                )
        }
    })
