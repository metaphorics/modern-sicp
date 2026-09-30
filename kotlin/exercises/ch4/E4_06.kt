// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.6

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.6: `let` as a derived expression over the kernel's `GLam`
 * and `GApp`. The rewrite turns the bindings into the parameters of a
 * lambda applied to the initializers, so the initializers evaluate in the
 * outer environment as operands do -- with x bound to 5 outside,
 * `let(x = 3, y = x)` reads y as 5, not 3. The rewritten form and the
 * `let` form answer the same value.
 *
 * Expected: the equivalence probe prints 7 twice; the body probe prints
 * 7; the outer-init probe prints 5; the shadow probe prints 2 then 5; the
 * nested probe prints 3; a malformed binding list is rejected at
 * admission with the `Syntax` category before any effect.
 */
public fun letEquivalenceTranscript(): String = throw PendingSolution()

/** The derived let computes the body in the new frame. => "7\n" */
public fun letBodyTranscript(): String = throw PendingSolution()

/** The inits evaluate in the outer environment. => "5\n" */
public fun letInitsOuterTranscript(): String = throw PendingSolution()

/** An inner let shadows and leaves the outer binding. => "2\n5\n" */
public fun letShadowTranscript(): String = throw PendingSolution()

/** Lets nest as derived expressions at every depth. => "3\n" */
public fun letNestedTranscript(): String = throw PendingSolution()

/** A malformed binding list fails typed at admission. => "Syntax" */
public fun malformedLetRejection(): String = throw PendingSolution()
