// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.31

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.31: laziness as an upward-compatible extension. The
 * declaration syntax `(name lazy)` / `(name lazy-memo)` rides the typed
 * parser; the evaluator binds strict parameters by evaluation, `lazy`
 * parameters as call-by-name thunks, and `lazy-memo` parameters as
 * call-by-need thunks, forcing at the demand sites.
 *
 * Expected answers: the mixed declaration `(f a (b lazy) c (d lazy-memo))`
 * prints `(1 5 5 4 30 30)` with count 5; the all-`lazy-memo` declaration
 * counts 4; a `lazy` parameter skips its dangerous argument, answering 7,
 * while a strict one dies with `Error: division by zero`.
 */
public fun annotatedMixedTranscript(): String = throw PendingSolution()

public fun annotatedAllMemoTranscript(): String = throw PendingSolution()

public fun lazyParamSkipsTranscript(): String = throw PendingSolution()

public fun strictParamEagerTranscript(): String = throw PendingSolution()
