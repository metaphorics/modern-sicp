// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.5

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.5: `when`-style clauses of the shape `test => recipient`.
 * Taking over the derived rewrite, an arrow clause rewrites to an
 * application of a one-parameter lambda over the test, so the test
 * evaluates exactly once and its value feeds the recipient -- the naive
 * `if(test, recipient(test), rest)` would evaluate the test twice. Plain
 * clauses chain as before, and no match with no fallback answers false.
 *
 * Expected: the lookup probe answers 2; the counting probe answers 2 and
 * then the recorded count 1; the recipient receives the value 99 doubled
 * to 198; plain clauses answer `second`; no match answers `false`.
 */
public fun arrowLookupTranscript(): String = throw PendingSolution()

/** The arrow test evaluates exactly once: the probe counts its runs.
 * => "2\n1\n" */
public fun arrowTestOnceTranscript(): String = throw PendingSolution()

/** The recipient receives the test's value, not its truth. => "198\n" */
public fun arrowValueTranscript(): String = throw PendingSolution()

/** Plain clauses still chain in order. => "second\n" */
public fun plainClausesTranscript(): String = throw PendingSolution()

/** No match with no fallback answers false. => "false\n" */
public fun noMatchTranscript(): String = throw PendingSolution()
