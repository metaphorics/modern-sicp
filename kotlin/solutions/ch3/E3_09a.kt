// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.9a

package sicp.ch3.exercises

import java.util.concurrent.atomic.AtomicReference

/**
 * How a small-stack run of one of the two factorials of exercise 3.9
 * ended: `Completed` with the value, or `Overflow` when the run died
 * with a `StackOverflowError`.
 */
public sealed interface StackOutcome {
    public data object Overflow : StackOutcome

    public data class Completed(
        val value: Long,
    ) : StackOutcome
}

/**
 * Runs `body` on a thread whose stack is `stackBytes` bytes and reports
 * how it ended: `Completed` with the value, or `Overflow` when the run
 * died with a `StackOverflowError`.
 */
private fun runOnSmallStack(
    stackBytes: Long,
    body: () -> Long,
): StackOutcome {
    val result = AtomicReference<StackOutcome>(StackOutcome.Overflow)
    val worker =
        Thread(
            null,
            {
                try {
                    result.set(StackOutcome.Completed(body()))
                } catch (e: StackOverflowError) {
                    result.set(StackOutcome.Overflow)
                }
            },
            "sicp-3-9a",
            stackBytes,
        )
    worker.start()
    worker.join()
    return result.get()
}

/** The recursive factorial of exercise 3.9, run on the small stack. */
public fun runRecursive(
    n: Long,
    stackBytes: Long,
): StackOutcome = runOnSmallStack(stackBytes) { factorial(n) }

/** The `tailrec` factorial of exercise 3.9, run on the small stack. */
public fun runTailrec(
    n: Long,
    stackBytes: Long,
): StackOutcome = runOnSmallStack(stackBytes) { factorialIter(n) }
