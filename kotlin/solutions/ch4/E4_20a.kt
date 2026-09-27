// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.20a

package sicp.ch4.solutions

// Exercise 4.20a (the edition addition chosen for Kotlin): nested `letrec`
// shadowing under the edition's scoping rule. While a `letrec`'s
// initializers run, the names it binds are already in scope -- only their
// values are still unassigned. An inner `letrec` that binds `f` again
// therefore shadows the outer recursive `f` from its first initializer
// onward, and an initializer that references `f` before the shadow's own
// `set!` runs reads the reserved inner binding and fails the typed
// premature-read check. Read with the outer binding in scope instead -- a
// discipline where a binding arrives only after its initializer, the way a
// plain `let` behaves -- the same reference names the outer recursive `f`,
// and the program recurses to the answer. Both readings run on the 4.20
// evaluator: the shadow program's inner form is the `letrec`, and the
// contrast program swaps exactly that form for a plain `let`.

/** The probe: the outer `letrec` binds `f` recursively; inside a recursive
 * call, an inner `letrec` binds `f` again and lists `k` first, so `k`'s
 * initializer references the shadowed `f` before the inner binding is
 * assigned. => Error: type mismatch: f is read before it is assigned */
private val SHADOW: String =
    """
    (letrec ((f (lambda (n)
                  (if (= n 0)
                      1
                      (letrec ((k (f (- n 1)))
                               (f (lambda (m) 0)))
                        k)))))
      (f 2))
    """.trimIndent()

/** The same program with the inner form a plain `let`: the initializer
 * reads the outer binding in scope, so the call recurses.
 * => 1 */
private val OUTER_SCOPE: String =
    """
    (letrec ((f (lambda (n)
                  (if (= n 0)
                      1
                      (let ((k (f (- n 1))))
                        k)))))
      (f 2))
    """.trimIndent()

/** The 4.20 evaluator on the shadow probe: the inner `f` is in scope and
 * unassigned while `k`'s initializer runs.
 * => "Error: type mismatch: f is read before it is assigned\n" */
public fun shadowedPrematureReadTranscript(): String = transcriptOn(::WithLetrec, SHADOW)

/** The inner form as a plain `let`: the reference finds the outer recursive
 * `f` and the program recurses. => "1\n" */
public fun outerScopeRecursionTranscript(): String = transcriptOn(::WithLetrec, OUTER_SCOPE)
