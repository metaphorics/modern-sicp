// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5_17

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_17Test :
    FunSpec({
        test("Exercise 5.17: every traced line is named by the label in effect") {
            labelTracedGcdTrace() shouldBe
                listOf(
                    "test-b: (test (op =) (reg b) (const 0))",
                    "test-b: (branch (label gcd-done))",
                    "test-b: (assign t (op rem) (reg a) (reg b))",
                    "test-b: (assign a (reg b))",
                    "test-b: (assign b (reg t))",
                    "test-b: (goto (label test-b))",
                    "test-b: (test (op =) (reg b) (const 0))",
                    "test-b: (branch (label gcd-done))",
                    "test-b: (assign t (op rem) (reg a) (reg b))",
                    "test-b: (assign a (reg b))",
                    "test-b: (assign b (reg t))",
                    "test-b: (goto (label test-b))",
                    "test-b: (test (op =) (reg b) (const 0))",
                    "test-b: (branch (label gcd-done))",
                    "test-b: (assign t (op rem) (reg a) (reg b))",
                    "test-b: (assign a (reg b))",
                    "test-b: (assign b (reg t))",
                    "test-b: (goto (label test-b))",
                    "test-b: (test (op =) (reg b) (const 0))",
                    "test-b: (branch (label gcd-done))",
                    "test-b: (assign t (op rem) (reg a) (reg b))",
                    "test-b: (assign a (reg b))",
                    "test-b: (assign b (reg t))",
                    "test-b: (goto (label test-b))",
                    "test-b: (test (op =) (reg b) (const 0))",
                    "test-b: (branch (label gcd-done))",
                )
        }
    })
