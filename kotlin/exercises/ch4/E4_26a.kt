// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.26a

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.26a (added by this edition): `when` as a derived expression,
 * the mirror of 4.26's `unless`. One macro-style rewrite to forms the
 * evaluator already has -- the `when` form lowers to
 * the `GIf` nest with a `GBlock` body and a `false` fallback -- pinned by a before/after
 * trace.
 *
 * Expected answers: before the derivation the name is unbound; after, the
 * true condition answers `yes`, the false condition answers `false`, and a
 * multi-expression body answers `3`.
 */
public fun whenBeforeTranscript(): String = throw PendingSolution()

public fun whenAfterTranscript(): String = throw PendingSolution()

public fun whenNoElseTranscript(): String = throw PendingSolution()

public fun whenBodySequenceTranscript(): String = throw PendingSolution()
