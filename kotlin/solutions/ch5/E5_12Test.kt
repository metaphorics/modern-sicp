// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_12

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_12Test :
    FunSpec({
        test("Exercise 5.12: the assembler's summary of the gcd machine") {
            gcdMachineSummary() shouldBe
                "(instructions (test (test (op =) (reg b) (const 0))) (branch (branch (label gcd-done)))" +
                " (assign (assign a (reg b)) (assign b (reg t)) (assign t (op rem) (reg a) (reg b)))" +
                " (goto (goto (label test-b))))\n" +
                "(registers a b t)\n" +
                "(entry-point registers )\n" +
                "(stack registers )\n" +
                "(sources (t ((op rem) (reg a) (reg b))) (a (reg b)) (b (reg t)))\n" +
                "(labels test-b gcd-done)"
        }
    })
