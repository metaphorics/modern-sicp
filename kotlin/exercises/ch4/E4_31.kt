// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.31

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.31: laziness as an upward-compatible extension. The
 * declaration syntax -- the call-by-name and call-by-need parameter kinds -- rides the typed
 * parser; the evaluator binds strict parameters by evaluation, the
 * call-by-name kind as unmemoized thunks, and the call-by-need kind as
 * memoized thunks, forcing at the demand sites.
 *
 * Expected answers: the mixed declaration (the call-by-name and call-by-need kinds mixed)
 * prints `[1, 5, 5, 4, 30, 30]` with count 5; the all-call-by-need
 * declaration counts 4; a call-by-name parameter skips its dangerous
 * argument, answering 7,
 * while a strict one raises `DivisionByZero`.
 */
public fun annotatedMixedTranscript(): String = throw PendingSolution()

public fun annotatedAllMemoTranscript(): String = throw PendingSolution()

public fun lazyParamSkipsTranscript(): String = throw PendingSolution()

public fun strictParamEagerTranscript(): String = throw PendingSolution()
