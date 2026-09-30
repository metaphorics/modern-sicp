// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.23

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

// Exercise 4.23: Alyssa P. Hacker does not understand why the text's
// sequence analysis needs to be so complicated, and her looping version --
// only the FIRST statement of a body analyzed at analysis time, the tail
// re-analyzed by the returned execution procedure at EXECUTION time --
// looks more efficient. Compare the two strategies over a two-statement
// body and a one-statement body.
//
// The values agree on both versions -- the probe defines a procedure
// whose body doubles its argument into a local and returns it, then calls
// it with 10 and with 30, answering 20 and 60 either way -- and the
// structural observation is in the analysis counts: the text version
// analyzes every body statement once at definition and nothing per call,
// while Alyssa's analyzes one fewer statement at definition and
// re-analyzes the tail on every execution.

/** One count of how many analysis passes a run makes: how many
 * statements were analyzed after the definition, and how many per call. */
public data class AnalysisProfile(
    /** Analysis passes while defining the probe procedure. */
    public val atDefinition: Int,
    /** Analysis passes while evaluating one call of it. */
    public val perCall: Int,
)

/** Both profiles for one analyzer: a two-statement body and a
 * one-statement body, the comparison 4.23 asks for. */
public data class SequenceProfiles(
    public val twoStatements: AnalysisProfile,
    public val oneStatement: AnalysisProfile,
)

/** The text analyzer on the value probe. => "20\n60\n" */
public fun textSequenceTranscript(): String = throw PendingSolution()

/** Alyssa's analyzer on the value probe: the same transcript.
 * => "20\n60\n" */
public fun alyssaSequenceTranscript(): String = throw PendingSolution()

/** The text analyzer's analysis counts. */
public fun textSequenceProfile(): SequenceProfiles = throw PendingSolution()

/** Alyssa's analyzer's analysis counts. */
public fun alyssaSequenceProfile(): SequenceProfiles = throw PendingSolution()
