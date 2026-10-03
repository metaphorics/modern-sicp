// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.7

package sicp.ch4.exercises

import sicp.runtime.PendingSolution

/**
 * Exercise 4.7: `let*` as nested `let`s. The rewrite folds the bindings
 * right to left -- `let*(x = 3, y = x + 1)` becomes
 * `let(x = 3) { let(y = x + 1) { body } }` -- so each initializer sees
 * the earlier bindings of the same `let*`. One derived-form clause is
 * sufficient: every nested `let` re-enters the kernel at the next depth,
 * which is also what makes a `let*` inside another's body work.
 *
 * Expected: the sequential-bindings probe prints 12; the chained probe
 * prints 8; the shadow probe prints 1; the nested probe prints 2.
 */
public fun sequentialBindingsTranscript(): String = throw PendingSolution()

/** Each initializer sees the earlier bindings of the same `let*`.
 * => "8\n" */
public fun chainedBindingsTranscript(): String = throw PendingSolution()

/** `let*` shadows from its first binding on. => "1\n" */
public fun letStarShadowTranscript(): String = throw PendingSolution()

/** A `let*` nested in another's body re-enters the clause. => "2\n" */
public fun letStarNestedTranscript(): String = throw PendingSolution()
