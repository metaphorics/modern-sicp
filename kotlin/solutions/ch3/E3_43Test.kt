// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.43

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.test.runTest

public class E3_43Test :
    FunSpec({
        test("Exercise 3.43: sequential exchanges preserve the multiset and the sum") {
            runTest {
                val (a, b, c) = sequentialExchanges(12)
                listOf(a, b, c).sorted() shouldBe listOf(10L, 20L, 30L)
                a + b + c shouldBe 60L
            }
        }

        test("Exercise 3.43: the forced broken schedule duplicates balances but preserves the sum") {
            runTest {
                val (a, b, c) = brokenConcurrentExchanges()
                listOf(a, b, c) shouldBe listOf(40L, 10L, 10L)
                a + b + c shouldBe 60L
            }
        }

        test("Exercise 3.43: serialized concurrent exchanges preserve the multiset") {
            runTest {
                val (a, b, c) = serializedConcurrentExchanges()
                listOf(a, b, c).sorted() shouldBe listOf(10L, 20L, 30L)
            }
        }
    })
