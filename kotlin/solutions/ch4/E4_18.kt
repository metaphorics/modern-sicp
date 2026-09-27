// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.18

package sicp.ch4.solutions

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList
import sicp.ch4.Evaluator
import sicp.runtime.AppE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.LambdaE
import sicp.runtime.LetBinding
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.Value
import sicp.runtime.VarE

/**
 * Exercise 4.18: the alternative scan-out strategy. Instead of 4.16's text
 * strategy -- reserve the names, then interleave `set!` with the rest of
 * the body -- this strategy first evaluates every initializer, and only
 * then assigns: the reserved names still hold `*unassigned*` while any
 * initializer runs, so an initializer that reads a fellow define fails the
 * typed premature-read check even where the text strategy would have
 * sailed through. The rewrites lower to one `let` of reservations whose
 * body applies a lambda to all the initializers at once: operand
 * evaluation is the "inner let binds every initializer first" of the
 * book's strategy (b), with generated parameter names (`init0`, ...) no
 * reader can produce.
 */
public fun scanOutDefinesAlt(body: List<Expr>): List<Expr> {
    val defines = body.filterIsInstance<DefineE>()
    if (defines.isEmpty()) return body
    val reserved = defines.map { LetBinding(it.name, LitE(unassignedMarker)) }.toPersistentList()
    val initParams = defines.indices.map { "init$it" }.toPersistentList()
    val assignments = defines.mapIndexed { index, define -> SetE(define.name, VarE("init$index")) }
    val rest = body.filterNot { it is DefineE }
    val initializersThenAssignments =
        AppE(
            LambdaE(initParams, null, (assignments + rest).toPersistentList()),
            defines.map { it.value }.toPersistentList(),
        )
    return listOf<Expr>(LetE(reserved, persistentListOf(initializersThenAssignments)))
}

/** The 4.18 evaluator: procedure bodies are built with the alternative scan applied. */
public class WithScanOutAlt(
    global: Env,
) : Evaluator(global) {
    override fun makeProcedure(
        params: PersistentList<String>,
        rest: String?,
        body: PersistentList<Expr>,
        env: Env,
        name: String?,
    ): Value = super.makeProcedure(params, rest, scanOutDefinesAlt(body).toPersistentList(), env, name)

    context(r: Raise<SchemeError>)
    override fun lookupVariable(
        name: String,
        env: Env,
    ): Value = requireAssigned(name, env.lookup(name))
}

/** The probe, in the shape of 3.5.4: `dy` defers its value behind a
 * zero-argument lambda, `y`'s initializer forces it eagerly.
 * => 3 under the text strategy; the premature read of `dy` under (b) */
private val PROBE: String =
    """
    (define (force thunk) (thunk))
    (define (demo)
      (define dy (lambda () 3))
      (define y (force dy))
      y)
    (demo)
    """.trimIndent()

/** The text strategy assigns in source order, so the forced read lands
 * after `dy` is set. => "3\n" */
public fun textStrategyTranscript(): String = transcriptOn(::WithScanOut, PROBE)

/** The alternative strategy evaluates `y`'s initializer before any
 * assignment, and the forced read hits the reservation.
 * => "Error: type mismatch: dy is read before it is assigned\n" */
public fun altStrategyTranscript(): String = transcriptOn(::WithScanOutAlt, PROBE)
