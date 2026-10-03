// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.30

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.30: does `eval-sequence` force? The text's rule evaluates
 * non-final expressions without forcing; Cy's rule forces them. Ben's
 * `forEach` prints the same under both, because `print` and `println`
 * are strict primitives whose operands force at application. Cy's `p2`
 * differs: under the text's rule its non-final `e` evaluates to the
 * delayed assignment without running it.
 *
 * Expected answers: `forEach` prints `\n57\n321\n88done` under both
 * rules; `p1(1)` is `[1, 2]` under both; `p2(1)` is `1` under the text's
 * rule and `[1, 2]` under Cy's.
 */
public fun forEachTextRuleTranscript(): String = throw PendingSolution()

public fun forEachCyRuleTranscript(): String = throw PendingSolution()

public fun p1P2TextRuleTranscript(): String = throw PendingSolution()

public fun p1P2CyRuleTranscript(): String = throw PendingSolution()
