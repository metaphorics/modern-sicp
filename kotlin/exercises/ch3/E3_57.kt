// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.57

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.57: how many additions are performed when we compute the
 * n-th Fibonacci number using the definition of fibs based on
 * add-streams? With the memoized tail each element past the first two
 * costs exactly one addition, never repeated, so the count at index n
 * is n - 1 (0 for n <= 1). With a plain lambda delay every tail force
 * re-runs its zip, whose two argument streams re-derive the whole tree
 * below them before their heads are added; the count then grows
 * exponentially, like the tree-recursive Fibonacci of section 1.2.2.
 * The memoized side counts through the pair stream the local builder
 * ties together, each element carrying the Fibonacci value and the
 * additions performed so far; the unmemoized side walks the same
 * construction cold, paying the fresh derivation of every element.
 */
public fun fibAdditions(
    n: Int,
    memoized: Boolean,
): Int = throw PendingSolution()
