// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.24

package sicp.ch4.exercises

import sicp.runtime.PendingSolution
import sicp.runtime.Value

// Exercise 4.24: measure the direct evaluator of 4.1.1 against the
// analyzer of 4.1.7 on tree recursion. Run the same `(fib 12)` program on
// both engines in fresh global environments, with warmup calls and then
// several timed batches, taking the median batch time of each. The TEST
// asserts only structural facts -- both engines compute the same value
// and both medians are positive -- because a timing assertion would flake
// on any other host; the measured ratio is reported in the `// =>`
// comment and the rationale, honestly, for one run on one machine.

/** One engine's benchmark reading: the agreed answer and the median batch
 * time in nanoseconds. */
public data class Timing(
    public val value: Value,
    public val medianNanos: Double,
)

/** The direct evaluator of 4.1.1: every evaluation re-dispatches on the
 * expression type. => a Timing with fib 12 and its median batch time */
public fun directEvaluatorTiming(): Timing = throw PendingSolution()

/** The analyzer of 4.1.7: the syntactic work runs once, at analysis time;
 * execution calls the stored execution procedures. => a Timing with fib 12
 * and its median batch time */
public fun analyzerTiming(): Timing = throw PendingSolution()
