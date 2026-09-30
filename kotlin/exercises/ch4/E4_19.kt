// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.19

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.19: the internal-definition scoping debate. The statement's
 * program binds an outer `a` of 1, defines `f` whose body defines `b` as
 * `a + x` and then redefines `a` as 5, and calls `f(10)`. Ben's
 * sequential rule initializes `b` against the outer `a` and answers 16.
 * Alyssa's 4.16 scan-out reserves every name first, so the read of `a` in
 * `b`'s initializer raises the typed premature-read fault. Eva's
 * simultaneous rule evaluates initializer values where the final values
 * of the defined names are visible, so `b` is 15 against the final `a` of
 * 5 and the program answers 20.
 *
 * Expected answers: 16 under the sequential rule; `UnassignedRead` under
 * the scan-out; 20 under the simultaneous rule.
 */
public fun benRuleTranscript(): String = throw PendingSolution()

/** Alyssa's scan-out raises the premature read. => "UnassignedRead" */
public fun alyssaRuleTranscript(): String = throw PendingSolution()

/** Eva's simultaneous rule answers 20. => "20\n" */
public fun evaRuleTranscript(): String = throw PendingSolution()
