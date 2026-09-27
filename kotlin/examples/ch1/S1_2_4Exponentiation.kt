// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.2.4

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/** The linear recursive process: Theta(n) steps and Theta(n) space. */
public fun exptRecursive(
    b: Long,
    n: Long,
): Long = if (n == 0L) 1L else b * exptRecursive(b, n - 1L)

private tailrec fun exptIter(
    b: Long,
    counter: Long,
    product: Long,
): Long = if (counter == 0L) product else exptIter(b, counter - 1L, b * product)

/** The linear iterative process: Theta(n) steps, Theta(1) space. */
public fun exptIterative(
    b: Long,
    n: Long,
): Long = exptIter(b, n, 1L)

private fun isEven(n: Long): Boolean = n % 2L == 0L

/**
 * Successive squaring: Theta(log n) steps and space, since each level of
 * recursion at most doubles the exponent it can dispatch on. `square`
 * here is the `Long`-valued one of section 1.1.4.
 */
public fun fastExpt(
    b: Long,
    n: Long,
): Long =
    when {
        n == 0L -> 1L
        isEven(n) -> square(fastExpt(b, n / 2L))
        else -> b * fastExpt(b, n - 1L)
    }

public class S1_2_4ExponentiationTest :
    FunSpec({
        test("all three processes agree on 2^10") {
            exptRecursive(2L, 10L) shouldBe 1024L
            exptIterative(2L, 10L) shouldBe 1024L
            fastExpt(2L, 10L) shouldBe 1024L
        }
        test("successive squaring also answers an odd exponent") {
            fastExpt(3L, 13L) shouldBe 1_594_323L
            fastExpt(3L, 13L) shouldBe exptRecursive(3L, 13L)
        }
    })
