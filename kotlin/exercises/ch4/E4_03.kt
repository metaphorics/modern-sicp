// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.3

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.3: the data-directed eval. Every compound clause lives in
 * the D19 registry under `(eval, node-kind)` -- the operation plus the
 * ordered tag list, absent option on a miss, overwrite on put -- and the
 * handlers recurse through the kernel's dispatch, so a later `put`
 * redirects the whole evaluator at every depth. Self-evaluating values
 * and variables stay residual cases (the 2.73 lesson), a node with no
 * entry falls back to the base clause chain, and the case-analysis probe
 * answers `yes` through the table.
 *
 * Expected: the case-analysis probe answers `yes`; the square probe
 * answers 49; the constructed data probe answers `[a, b]` (the
 * constructor lesson where the old language quoted); the residual probe
 * answers 19; the installed-clause probe answers `ran` then `no`; the
 * overwritten-clause probe answers `replaced`.
 */
public fun caseThroughTableTranscript(): String = throw PendingSolution()

/** Definitions and applications run through table clauses. => "49\n" */
public fun applicationThroughTableTranscript(): String = throw PendingSolution()

/** Constructor data denotes itself through the table. => "[a, b]\n" */
public fun constructedDataTranscript(): String = throw PendingSolution()

/** Self-evaluating values and variables stay residual cases. => "19\n" */
public fun residualDispatchTranscript(): String = throw PendingSolution()

/** A clause installed after construction extends the language.
 * => "ran\nno\n" */
public fun installedClauseTranscript(): String = throw PendingSolution()

/** A later put overwrites an installed clause. => "replaced\n" */
public fun overwrittenClauseTranscript(): String = throw PendingSolution()
