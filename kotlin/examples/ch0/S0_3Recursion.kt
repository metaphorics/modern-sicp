// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** Plain recursion: every call takes a stack frame. */
public fun factorial(n: Long): Long = if (n <= 1L) 1L else n * factorial(n - 1L)

/**
 * The same linear process expressed iteratively: `tailrec` compiles the
 * self-call into a loop, so the stack never grows.
 */
public tailrec fun factorialIter(
    n: Long,
    acc: Long = 1L,
): Long = if (n <= 1L) acc else factorialIter(n - 1L, acc * n)

/** A `while` states the loop directly; the book's 1.2 counters look like this. */
public fun sumTo(n: Long): Long {
    var total = 0L
    var k = 1L
    while (k <= n) {
        total += k
        k += 1
    }
    return total
}

/** Tree recursion: [fib] calls itself twice per step. */
public fun fib(n: Int): Long = if (n < 2) n.toLong() else fib(n - 1) + fib(n - 2)

public class S0_3RecursionTest :
    FunSpec({
        test("recursive and iterative factorial agree") {
            factorial(10) shouldBe 3_628_800L
            factorialIter(10) shouldBe 3_628_800L
            factorial(20) shouldBe 2_432_902_008_176_640_000L
            factorialIter(20) shouldBe 2_432_902_008_176_640_000L
        }
        test("a while loop carries its own state") {
            sumTo(100) shouldBe 5_050L
        }
        test("tree recursion fans out") {
            fib(10) shouldBe 55L
        }
    })
