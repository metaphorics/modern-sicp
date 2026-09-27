// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.9a

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 3.9a: the two factorials of exercise 3.9 differ in one
 * observable way -- the recursive one needs one stack frame per pending
 * multiplication, the `tailrec` one needs a constant number. Run each
 * version on a thread whose stack is only `stackBytes` bytes and report
 * the outcome: `Overflow` when the run dies with a `StackOverflowError`,
 * `Completed` with the value when it answers. Then explain, in terms of
 * frames and enclosing environments, what the error says about the
 * recursive version's structure, why no `n` can kill the `tailrec`
 * version, and what Kotlin checks before accepting the `tailrec` mark.
 */
public sealed interface StackOutcome {
    public data object Overflow : StackOutcome

    public data class Completed(
        val value: Long,
    ) : StackOutcome
}

public fun runRecursive(
    n: Long,
    stackBytes: Long,
): StackOutcome = throw PendingSolution()

public fun runTailrec(
    n: Long,
    stackBytes: Long,
): StackOutcome = throw PendingSolution()
