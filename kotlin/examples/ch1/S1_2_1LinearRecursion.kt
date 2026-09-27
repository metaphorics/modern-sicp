// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.2.1

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * The linear recursive process: each call defers a multiplication until the
 * recursive call returns, so the chain of pending multiplications grows
 * with [n]. This self-call is not in tail position, so Kotlin's compiler
 * would reject a `tailrec` marker on this function.
 */
public fun factorialRecursive(n: Long): Long = if (n == 1L) 1L else n * factorialRecursive(n - 1L)

/**
 * The footnote's block-structured version: [iter] is a local function, so
 * its name and the state variables it threads stay invisible outside
 * [factorialBlockStructured], exactly as the book's internal `define` does.
 */
public fun factorialBlockStructured(n: Long): Long {
    tailrec fun iter(
        product: Long,
        counter: Long,
    ): Long = if (counter > n) product else iter(product * counter, counter + 1L)
    return iter(1L, 1L)
}

/**
 * The linear iterative process: `product`, `counter`, and `maxCount`
 * completely describe the state at every step, so the self-call in tail
 * position compiles to a loop under `tailrec`.
 */
public tailrec fun factIter(
    product: Long,
    counter: Long,
    maxCount: Long,
): Long = if (counter > maxCount) product else factIter(counter * product, counter + 1L, maxCount)

/** Named apart from [factorialRecursive] so both processes can coexist in one file. */
public fun factorialIterative(n: Long): Long = factIter(1L, 1L, n)

public class S1_2_1LinearRecursionTest :
    FunSpec({
        test("all three processes compute 6!, the session of Figure 1.3 and 1.4") {
            factorialRecursive(6L) shouldBe 720L
            factorialBlockStructured(6L) shouldBe 720L
            factorialIterative(6L) shouldBe 720L
        }
        test("the iterative process still answers a larger n") {
            factorialIterative(20L) shouldBe 2_432_902_008_176_640_000L
            factorialRecursive(20L) shouldBe factorialIterative(20L)
        }
    })
