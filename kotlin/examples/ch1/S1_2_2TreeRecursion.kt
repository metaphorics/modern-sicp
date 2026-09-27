// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, section 1.2.2

package sicp.ch1.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

/**
 * The tree-recursive process of Figure 1.5: two calls per invocation
 * (except at the leaves), so no rewrite of this self-call is ever in tail
 * position. Tree recursion has no iterative twin at the syntax level; it
 * stays plain recursion in every edition.
 */
public fun fibRecursive(n: Long): Long =
    when {
        n == 0L -> 0L
        n == 1L -> 1L
        else -> fibRecursive(n - 1L) + fibRecursive(n - 2L)
    }

private tailrec fun fibIter(
    a: Long,
    b: Long,
    count: Long,
): Long = if (count == 0L) b else fibIter(a + b, a, count - 1L)

/** The linear iterative process: one pass, no chain of deferred sums. */
public fun fibIterative(n: Long): Long = fibIter(1L, 0L, n)

private fun firstDenomination(kindsOfCoins: Int): Long =
    when (kindsOfCoins) {
        1 -> 1L
        2 -> 5L
        3 -> 10L
        4 -> 25L
        else -> 50L
    }

private fun cc(
    amount: Long,
    kindsOfCoins: Int,
): Long =
    when {
        amount == 0L -> 1L
        amount < 0L || kindsOfCoins == 0 -> 0L
        else -> cc(amount, kindsOfCoins - 1) + cc(amount - firstDenomination(kindsOfCoins), kindsOfCoins)
    }

/** The number of ways to change [amount], counting half-dollars down to pennies, as five kinds of coins. */
public fun countChange(amount: Long): Long = cc(amount, 5)

public class S1_2_2TreeRecursionTest :
    FunSpec({
        test("the tree-recursive Fibonacci matches the sequence Figure 1.5 draws") {
            (0L..5L).map { fibRecursive(it) } shouldBe listOf(0L, 1L, 1L, 2L, 3L, 5L)
        }
        test("the iterative Fibonacci agrees with the tree-recursive one everywhere it is cheap to check") {
            (0L..20L).map { fibIterative(it) } shouldBe (0L..20L).map { fibRecursive(it) }
        }
        test("changing a dollar takes 292 ways, as the text reports") {
            countChange(100L) shouldBe 292L
        }
    })
