// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.9

package sicp.ch3.exercises

/**
 * The recursive version: each call opens a frame binding its own `n` and
 * waits for the recursive answer, so `factorial(6)` builds a stack of
 * frames six deep before the multiplications start.
 */
public fun factorial(n: Long): Long = if (n <= 1L) 1L else n * factorial(n - 1L)

/**
 * The book's `fact-iter`: the state lives in parameters, the self-call is
 * the last thing the body does, and the `tailrec` mark makes the compiler
 * rewrite it as a loop, so the whole run shares one frame.
 */
public tailrec fun factIter(
    product: Long,
    counter: Long,
    maxCount: Long,
): Long = if (counter > maxCount) product else factIter(counter * product, counter + 1, maxCount)

public fun factorialIter(n: Long): Long = factIter(1L, 1L, n)
