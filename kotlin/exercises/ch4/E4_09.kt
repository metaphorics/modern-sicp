// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
// Chapter 4, exercise 4.9

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.9: iteration constructs designed as derived expressions.
 * `while` and `until` bind a self-calling procedure with `GLetRec` and make
 * its initial call with `GApp`; the procedure tests whether to run the body
 * or return. Its recursive calls use the evaluator's host stack, so it
 * makes no constant-stack claim. The probes: a while
 * summing 1 to 5 answers 15; a false while and an initially satisfied until
 * leave their bodies untouched; an until product answers 95040 at count 13;
 * nested loops count 6.
 */
public fun whileSumTranscript(): String = throw PendingSolution()

/** A while whose test is false never runs its body. => "0\n" */
public fun whileNeverRunsTranscript(): String = throw PendingSolution()

/** The until product and its counter. => "95040\n13\n" */
public fun untilProductTranscript(): String = throw PendingSolution()

/** An until whose test holds never runs its body. => "0\n" */
public fun untilNeverRunsTranscript(): String = throw PendingSolution()

/** Loops nest, each wrapper block holding its own loop state. => "6\n" */
public fun nestedLoopsTranscript(): String = throw PendingSolution()
