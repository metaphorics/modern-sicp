// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.26: Ben's side derives `unless` as a special form lowering to
 * the swapped `if`, which keeps the untouched arm unevaluated but makes
 * the name syntax, not a value. Alyssa's side keeps `unless` an ordinary
 * procedure under the lazy evaluator, whose thunk arms compose with `map`
 * and `apply` because the unchosen arm is never forced.
 *
 * Expected answers: the derived `unless` answers 42 on the armed call and
 * fails `null` as a value; the lazy procedure
 * answers 42, maps to `[42, 7]`, and applies to 7.
 */
public fun unlessDerivedTranscript(): String = throw PendingSolution()

public fun unlessDerivedValueUseTranscript(): String = throw PendingSolution()

public fun unlessLazyProcedureTranscript(): String = throw PendingSolution()

public fun unlessLazyMappedTranscript(): String = throw PendingSolution()

public fun unlessLazyApplyTranscript(): String = throw PendingSolution()
