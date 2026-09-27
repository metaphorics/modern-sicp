// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20a

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.20a (the edition addition chosen for Kotlin): nested `letrec`
 * shadowing under the edition's scoping rule. While a `letrec`'s
 * initializers run, the names it binds are already in scope -- only their
 * values are still unassigned. An inner `letrec` that binds `f` again
 * therefore shadows the outer recursive `f` from its first initializer
 * onward, and an initializer that references `f` before the shadow's own
 * `set!` runs reads the reserved inner binding and fails the typed
 * premature-read check. Read with the outer binding in scope instead -- a
 * discipline where a binding arrives only after its initializer, the way a
 * plain `let` behaves -- the same reference names the outer recursive `f`
 * and the program recurses to the answer.
 *
 * Expected answers: the shadow program fails with `Error: type mismatch: f
 * is read before it is assigned`; the same program with the inner form a
 * plain `let` recurses and answers 1.
 */
public fun shadowedPrematureReadTranscript(): String = throw PendingSolution()

public fun outerScopeRecursionTranscript(): String = throw PendingSolution()
