// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.9

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.9: iteration constructs designed as derived expressions.
 * `while(test) { body }` and `until(test) { body }` desugar to the
 * kernel's `GWhileStmt` under a `GBlock`, with the loop state in `GVarStmt`
 * bindings of an enclosing block. The rewrite fixes both properties that
 * matter: the body re-enters the full evaluator at every iteration, and
 * the loop runs an iterative process in constant host stack. The probes:
 * a while summing 1 to 5 answers 15; a while whose test is false never
 * runs its body; an until product answers 95040 and its counter 13; an
 * until whose test holds never runs its body; the hundred-thousand-trip
 * loop completes; nested loops count 6.
 */
public fun whileSumTranscript(): String = throw PendingSolution()

/** A while whose test is false never runs its body. => "0\n" */
public fun whileNeverRunsTranscript(): String = throw PendingSolution()

/** The until product and its counter. => "95040\n13\n" */
public fun untilProductTranscript(): String = throw PendingSolution()

/** An until whose test holds never runs its body. => "0\n" */
public fun untilNeverRunsTranscript(): String = throw PendingSolution()

/** The iterative loop runs in constant host stack. => "100000\n" */
public fun iterativeLoopTranscript(): String = throw PendingSolution()

/** Loops nest, each wrapper block holding its own loop state. => "6\n" */
public fun nestedLoopsTranscript(): String = throw PendingSolution()
