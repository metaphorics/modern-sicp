// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.23

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

// Exercise 4.23: Alyssa P. Hacker does not understand why
// `analyze-sequence` needs to be so complicated, and her looping
// `execute-sequence` -- only the FIRST expression of a sequence analyzed
// at analysis time, the tail re-analyzed by the returned closure at
// EXECUTION time -- looks more efficient. Override the analyzer's
// `analyzeSequence` seam with her version (one-expression bodies return
// the body's own execution procedure; multi-expression bodies analyze the
// first expression and defer the rest to each execution), then compare
// against the text version for one-expression and two-expression bodies.
//
// The values agree on both versions -- the probe `(define (f x)
// (set! x (* x 2)) x)` then `(f 10)`, `(f 30)` answers 20 and 60 either
// way -- and the structural observation is in the analysis counts: the
// text version analyzes every body expression once at definition and
// nothing per call, while Alyssa's analyzes one fewer expression at
// definition and re-analyzes the tail on every execution.

/** One count of how many `analyze` calls a run makes: how many
 * expressions were analyzed after the definition, and how many per call. */
public data class AnalysisProfile(
    /** Hook calls while defining the probe procedure. */
    public val atDefinition: Int,
    /** Hook calls while evaluating one call of it. */
    public val perCall: Int,
)

/** Both profiles for one analyzer: a two-expression body and a
 * one-expression body, the comparison 4.23 asks for. */
public data class SequenceProfiles(
    public val twoExpressions: AnalysisProfile,
    public val oneExpression: AnalysisProfile,
)

/** The text analyzer on the value probe. => 20, then 60 */
public fun textSequenceTranscript(): String = throw PendingSolution()

/** Alyssa's analyzer on the value probe: the same transcript. => 20, then 60 */
public fun alyssaSequenceTranscript(): String = throw PendingSolution()

/** The text analyzer's analysis counts. */
public fun textSequenceProfile(): SequenceProfiles = throw PendingSolution()

/** Alyssa's analyzer's analysis counts. */
public fun alyssaSequenceProfile(): SequenceProfiles = throw PendingSolution()
