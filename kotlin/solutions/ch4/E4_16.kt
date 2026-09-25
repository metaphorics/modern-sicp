// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.16

package sicp.ch4.solutions

import arrow.core.raise.Raise
import arrow.core.raise.either
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.toPersistentList
import sicp.ch4.Evaluator
import sicp.ch4.OutputSink
import sicp.ch4.formatError
import sicp.ch4.parseProgram
import sicp.ch4.printValue
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.LetBinding
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.VSym
import sicp.runtime.Value

// Exercise 4.16: scan out internal definitions. Alyssa's `scan-out-defines`
// rewrites a procedure body that contains `define` forms into one `let`
// reserving every defined name with the section's `*unassigned*` symbol,
// followed by the assignments in source order and the rest of the body; a
// body without internal defines passes through unchanged. Installed in
// `make-procedure`, the scan runs once per procedure creation, so every
// application executes the already-rewritten body. Reading a name that
// still carries the marker is the typed premature-read fault: this edition
// picks `TypeMismatch`, because the binding exists (a plain `Unbound`
// lookup would mean something else) and what failed is the read of a value
// no proper operation accepts.

/** The section's `*unassigned*` marker: the reserved value of a name whose `set!` has not run yet. */
public val unassignedMarker: Value = VSym("*unassigned*")

/** The premature-read check behind [WithScanOut.lookupVariable] and the letrec evaluators. */
context(r: Raise<SchemeError>)
public fun requireAssigned(
    name: String,
    value: Value,
): Value {
    if (value == unassignedMarker) {
        r.raise(SchemeError.TypeMismatch("$name is read before it is assigned"))
    }
    return value
}

/**
 * Alyssa's `scan-out-defines` over a parsed body: the top-level `define`
 * forms become `let` reservations plus `set!` assignments in source order,
 * and the remaining body expressions follow. Nested lambdas keep their own
 * defines; each gets scanned when its own procedure is made.
 */
public fun scanOutDefines(body: List<Expr>): List<Expr> {
    val defines = body.filterIsInstance<DefineE>()
    if (defines.isEmpty()) return body
    val reserved = defines.map { LetBinding(it.name, LitE(unassignedMarker)) }.toPersistentList()
    val assignments = defines.map { SetE(it.name, it.value) }
    val rest = body.filterNot { it is DefineE }
    return listOf<Expr>(LetE(reserved, (assignments + rest).toPersistentList()))
}

/** The 4.16 evaluator: procedures are built with their body already scanned. */
public class WithScanOut(
    global: Env,
) : Evaluator(global) {
    override fun makeProcedure(
        params: PersistentList<String>,
        rest: String?,
        body: PersistentList<Expr>,
        env: Env,
        name: String?,
    ): Value = super.makeProcedure(params, rest, scanOutDefines(body).toPersistentList(), env, name)

    context(r: Raise<SchemeError>)
    override fun lookupVariable(
        name: String,
        env: Env,
    ): Value = requireAssigned(name, env.lookup(name))
}

/** Runs [text] on [evaluatorFactory]'s evaluator and returns the printer-
 * contract transcript: defines print nothing, values print one line each. */
internal fun transcriptOn(
    evaluatorFactory: (Env) -> Evaluator,
    text: String,
    install: ((Env) -> Unit)? = null,
): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    install?.invoke(env)
    val evaluator = evaluatorFactory(env)
    either {
        for (expr in parseProgram(readProgram(text))) {
            if (expr is DefineE) {
                evaluator.eval(expr, env) // a define prints nothing
                continue
            }
            sink.line(printValue(evaluator.eval(expr, env)))
        }
    }.fold(
        { e -> sink.line("Error: ${formatError(e)}") },
        { },
    )
    return sink.toString()
}

/** The statement's mutual recursion: each internal `define` names a lambda,
 * so the scan reserves both names and the calls find them assigned.
 * => #t */
private val MUTUAL: String =
    """
    (define (f n)
      (define (even? k) (if (= k 0) true (odd? (- k 1))))
      (define (odd? k) (if (= k 0) false (even? (- k 1))))
      (even? n))
    (f 10)
    """.trimIndent()

/** The premature read: `b`'s initializer reads `a`, whose `set!` runs later.
 * => Error: type mismatch: a is read before it is assigned */
private val PREMATURE: String =
    """
    (define (g)
      (define b (+ a 1))
      (define a 5)
      b)
    (g)
    """.trimIndent()

/** The mutual-recursion program under the scan-out. => "#t\n" */
public fun mutualRecursionTranscript(): String = transcriptOn(::WithScanOut, MUTUAL)

/** The premature read under the scan-out. => "Error: type mismatch: a is read before it is assigned\n" */
public fun prematureReadTranscript(): String = transcriptOn(::WithScanOut, PREMATURE)

/** The same program on the base evaluator: the sequential rule fails the
 * eager initializer as a plain unbound variable. => "Error: unbound variable: a\n" */
public fun basePrematureTranscript(): String = transcriptOn(::Evaluator, PREMATURE)
