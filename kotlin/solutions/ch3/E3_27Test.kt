// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.27

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E3_27Test :
    FunSpec({
        test("the book's session: memoFib gives the Fibonacci numbers") {
            val memoFib = makeMemoFib()
            memoFib(0L) shouldBe 0L
            memoFib(1L) shouldBe 1L
            memoFib(3L) shouldBe 2L
            memoFib(10L) shouldBe 55L
            memoFib(30L) shouldBe 832_040L
            memoFib(50L) shouldBe 12_586_269_025L
        }

        test("the trace of memoFib(3): four computes, then the table answers alone") {
            var memoFib: ((Long) -> Long)? = null
            var computes = 0

            fun fibStep(n: Long): Long {
                val recurse =
                    memoFib
                        ?: error("memoFib consulted before the test finished building it")
                computes++
                return if (n == 0L) {
                    0L
                } else if (n == 1L) {
                    1L
                } else {
                    recurse(n - 1) + recurse(n - 2)
                }
            }
            val built = memoize(::fibStep)
            memoFib = built

            built(3L) shouldBe 2L
            computes shouldBe 4

            built(3L) shouldBe 2L
            computes shouldBe 4
        }

        test("the linear step count: memoFib(20) computes exactly 21 times") {
            var memoFib: ((Long) -> Long)? = null
            var computes = 0

            fun fibStep(n: Long): Long {
                val recurse =
                    memoFib
                        ?: error("memoFib consulted before the test finished building it")
                computes++
                return if (n == 0L) {
                    0L
                } else if (n == 1L) {
                    1L
                } else {
                    recurse(n - 1) + recurse(n - 2)
                }
            }
            val built = memoize(::fibStep)
            memoFib = built

            built(20L) shouldBe 6_765L
            computes shouldBe 21
        }

        test("memoizing the plain fib would still explode inside the first compute") {
            var calls = 0

            fun fibPlain(n: Long): Long {
                calls++
                return if (n == 0L) {
                    0L
                } else if (n == 1L) {
                    1L
                } else {
                    fibPlain(n - 1) + fibPlain(n - 2)
                }
            }
            val memoizedPlain = memoize(::fibPlain)
            memoizedPlain(20L) shouldBe 6_765L
            (calls > 10_000) shouldBe true
        }
    })
