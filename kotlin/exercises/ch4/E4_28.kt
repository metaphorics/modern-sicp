// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.28

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.28: why the operator forces. `apply` dispatches on the
 * procedure value in the operator position; `((id +) 2 3)` shows the
 * forced operator answering 5, and the counterfactual evaluator that
 * applies whatever `eval` returned without forcing fails on the thunk.
 *
 * Expected answers: `5` with the forcing, `null` without it.
 */
public fun forcedOperatorTranscript(): String = throw PendingSolution()

public fun unforcedOperatorTranscript(): String = throw PendingSolution()
