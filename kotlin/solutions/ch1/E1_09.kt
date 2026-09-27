// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.9

package sicp.ch1.exercises

/**
 * The recursive process: the self-call sits inside `.inc()`, so a chain of
 * deferred increments builds up exactly as `factorialRecursive`'s chain of
 * deferred multiplications does. The compiler rejects a `tailrec` marker
 * here, because the call is not in tail position.
 */
public fun plusRecursive(
    a: Long,
    b: Long,
): Long = if (a == 0L) b else plusRecursive(a.dec(), b).inc()

/**
 * The iterative process: the self-call is the entire body of the else
 * branch, in tail position, so `tailrec` compiles it into a loop.
 */
public tailrec fun plusIterative(
    a: Long,
    b: Long,
): Long = if (a == 0L) b else plusIterative(a.dec(), b.inc())

public fun ex_1_09(): Pair<Long, Long> = plusRecursive(4L, 5L) to plusIterative(4L, 5L)
