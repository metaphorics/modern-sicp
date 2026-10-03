// SPDX-License-Identifier: GPL-3.0-only
// Shared engine-observation helpers for the chapter-4 solutions: the one
// pinned outcome rendering every example/test boundary uses.

package sicp.ch4.solutions

import arrow.core.Either
import sicp.guest.AdmissionError
import sicp.guest.RunResult

/**
 * The pinned observation text of one engine outcome: the ordered output
 * stream, then the typed fault as its category. Diagnostic wording is never
 * compared across implementations (grammar 3.8), so a fault renders exactly
 * its category, and an admission failure prints its rejection category and
 * has executed nothing.
 */
internal fun outcomeText(outcome: Either<AdmissionError, RunResult>): String =
    outcome.fold({ e -> "Error: ${e.category}\n" }, { result -> resultText(result) })

/** The observation text of one completed run: output, then a fault line. */
internal fun resultText(result: RunResult): String = result.output + (result.error?.let { "Error: ${it.category}\n" } ?: "")
