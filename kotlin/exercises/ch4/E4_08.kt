// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.8

package sicp.ch4.exercises

import arrow.core.raise.Raise
import sicp.ch4.EvalStep
import sicp.ch4.Evaluator
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.PendingSolution
import sicp.runtime.SchemeError
import sicp.runtime.Value

/**
 * Exercise 4.8: named `let`. The grammar's parser rejects the named shape
 * `(let name bindings body)` -- a symbol in the bindings slot -- so
 * [namedLetExpansion] renames it to `named-let` at the datum level, and the
 * parser delivers that as the application the step seam expects.
 * [namedLetToCombination] derives the form to the set!-based wrapper
 * `((lambda (name) (set! name (lambda bindings body)) (name inits))
 * '*named-let*)`: the wrapper's frame binds `name`, the `set!` installs the
 * loop procedure into that binding, and the body's recursive self-calls
 * resolve it there. The book's named-let Fibonacci answers 55.
 */
public class WithNamedLet(
    global: Env,
) : Evaluator(global) {
    context(r: Raise<SchemeError>)
    override fun step(
        expr: Expr,
        env: Env,
    ): EvalStep = throw PendingSolution()
}

/** The datum-level parse seam: `(let name bindings body...)` becomes
 * `(named-let name bindings body...)`, recursively; quoted data passes
 * through untouched. */
public fun namedLetExpansion(datum: Value): Value = throw PendingSolution()

/** The book's named-let derivation: the set!-based wrapper application. */
context(r: Raise<SchemeError>)
public fun namedLetToCombination(expr: AppE): Expr = throw PendingSolution()
