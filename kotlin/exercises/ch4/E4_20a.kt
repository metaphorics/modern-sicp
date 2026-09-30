// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20a

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.20a (the edition addition chosen for Kotlin): nested
 * `GLetRec` shadowing under the edition's scoping rule. While a recursive
 * binding's initializers run, the names it binds are already in scope --
 * only their values are still unassigned. An inner `GLetRec` that binds
 * `f` again therefore shadows the outer recursive `f` from its first
 * initializer onward, and an initializer that references `f` before the
 * shadow's own assignment runs reads the reserved inner binding and
 * raises the typed premature-read fault. Read with the outer binding in
 * scope instead -- a discipline where a binding arrives only after its
 * initializer, the way a plain `GLet` behaves -- the same reference names
 * the outer recursive `f` and the program recurses to the answer.
 *
 * Expected answers: the shadow program raises `UnassignedRead`; the same
 * program with the inner form a plain `GLet` recurses and answers 1.
 */
public fun shadowedPrematureReadTranscript(): String = throw PendingSolution()

/** The same program with the inner form a plain `GLet`. => "1\n" */
public fun outerScopeRecursionTranscript(): String = throw PendingSolution()
