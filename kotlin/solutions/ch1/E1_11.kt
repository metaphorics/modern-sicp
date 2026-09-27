// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.11

package sicp.ch1.exercises

/** The tree-recursive process: three calls per invocation once past the base cases. */
public fun fRecursive(n: Long): Long =
    if (n < 3L) {
        n
    } else {
        fRecursive(n - 1L) + 2L * fRecursive(n - 2L) + 3L * fRecursive(n - 3L)
    }

private tailrec fun fIter(
    a: Long,
    b: Long,
    c: Long,
    count: Long,
): Long = if (count == 0L) a else fIter(a + 2L * b + 3L * c, a, b, count - 1L)

/**
 * The iterative process: `a`, `b`, `c` slide the window of the three
 * preceding values forward, starting from `f(2), f(1), f(0)`.
 */
public fun fIterative(n: Long): Long = if (n < 3L) n else fIter(2L, 1L, 0L, n - 2L)

public fun ex_1_11(n: Long): Pair<Long, Long> = fRecursive(n) to fIterative(n)
