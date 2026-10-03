// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.8: tests.

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_08Test :
    FunSpec({
        test("Exercise 4.8: named-let Fibonacci computes the book's value") {
            namedLetFibonacciTranscript() shouldBe "55\n"
        }

        test("Exercise 4.8: the recursive name remains local") {
            loopNameLocalTranscript() shouldBe "1\n7\n"
        }

        test("Exercise 4.8: ordinary let still evaluates") {
            plainLetTranscript() shouldBe "3\n"
        }
    })
