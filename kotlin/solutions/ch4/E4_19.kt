// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.19

package sicp.ch4.solutions

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.toPersistentList
import sicp.ch4.Evaluator
import sicp.runtime.AppE
import sicp.runtime.BeginE
import sicp.runtime.CondClause
import sicp.runtime.CondE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.IfE
import sicp.runtime.LambdaE
import sicp.runtime.LetBinding
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.QuoteE
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.Value
import sicp.runtime.VarE

// Exercise 4.19: the internal-definition scoping debate. One program, three
// disciplines: Ben's sequential rule runs it on the plain evaluator and
// answers 16, because `b`'s initializer reads the outer `a` of 1; Alyssa's
// 4.16 scan-out reserves every name first, so the same read fails the typed
// premature-read check; Eva's simultaneous rule reserves the names and
// emits the assignments in eager-read dependency order, so `b` is
// initialized against the final `a` of 5 and the program answers 20. The
// dependency order comes from an occurrence check that collects the
// variables an initializer reads eagerly -- a read inside a lambda defers
// until the lambda is called, so mutual recursion stays order-free. A
// genuinely cyclic pair of initializers falls back to source order, where
// the reservation check reports the deadlock instead of looping.

/** The statement's program. => 16 sequential, premature read under the scan-out, 20 simultaneous */
private val DEBATE: String =
    """
    (let ((a 1))
      (define (f x)
        (define b (+ a x))
        (define a 5)
        (+ a b))
      (f 10))
    """.trimIndent()

/** The variables [expr] reads eagerly: [VarE] occurrences outside any
 * [LambdaE], since a lambda body's reads defer until the call. */
private fun eagerReads(expr: Expr): Set<String> {
    val names = mutableSetOf<String>()
    walkReads(expr, names)
    return names
}

private fun walkReads(
    expr: Expr,
    into: MutableSet<String>,
) {
    when (expr) {
        is VarE -> {
            into.add(expr.name)
        }

        is LitE, is QuoteE -> {}

        is LambdaE -> {}

        // a lambda body's reads defer until the lambda is called
        is IfE -> {
            walkReads(expr.predicate, into)
            walkReads(expr.consequent, into)
            walkReads(expr.alternative, into)
        }

        is SetE -> {
            walkReads(expr.value, into)
        }

        is DefineE -> {
            walkReads(expr.value, into)
        }

        is BeginE -> {
            expr.actions.forEach { walkReads(it, into) }
        }

        is CondE -> {
            expr.clauses.forEach { clause ->
                when (clause) {
                    is CondClause.Clause -> {
                        walkReads(clause.test, into)
                        clause.body.forEach { walkReads(it, into) }
                    }

                    is CondClause.Else -> {
                        clause.body.forEach { walkReads(it, into) }
                    }
                }
            }
        }

        is LetE -> {
            expr.bindings.forEach { walkReads(it.value, into) }
            expr.body.forEach { walkReads(it, into) }
        }

        is AppE -> {
            walkReads(expr.operator, into)
            expr.operands.forEach { walkReads(it, into) }
        }
    }
}

/** The scanned assignments in dependency order: a define whose initializer
 * eagerly reads fellow defines waits until those are assigned; ties and
 * cycles keep source order. */
private fun orderedAssignments(defines: List<DefineE>): List<SetE> {
    val defined = defines.map { it.name }.toSet()
    val pending =
        defines
            .map { it to (eagerReads(it.value) intersect defined) }
            .toMutableList()
    val assigned = mutableSetOf<String>()
    val ordered = mutableListOf<SetE>()
    while (pending.isNotEmpty()) {
        val ready = pending.indexOfFirst { it.second.all { name -> name in assigned } }
        val index = if (ready >= 0) ready else 0 // a genuine cycle: source order, the marker reports the deadlock
        val (define, _) = pending.removeAt(index)
        ordered.add(SetE(define.name, define.value))
        assigned.add(define.name)
    }
    return ordered
}

private fun scanOutSimultaneous(body: List<Expr>): List<Expr> {
    val defines = body.filterIsInstance<DefineE>()
    if (defines.isEmpty()) return body
    val reserved = defines.map { LetBinding(it.name, LitE(unassignedMarker)) }.toPersistentList()
    val rest = body.filterNot { it is DefineE }
    return listOf<Expr>(LetE(reserved, (orderedAssignments(defines) + rest).toPersistentList()))
}

/** Eva's evaluator: the scanned body assigns in eager-read dependency order,
 * so every initializer sees the final value of the names it reads. */
public class EvaSimultaneous(
    global: Env,
) : Evaluator(global) {
    override fun makeProcedure(
        params: PersistentList<String>,
        rest: String?,
        body: PersistentList<Expr>,
        env: Env,
        name: String?,
    ): Value = super.makeProcedure(params, rest, scanOutSimultaneous(body).toPersistentList(), env, name)

    context(r: Raise<SchemeError>)
    override fun lookupVariable(
        name: String,
        env: Env,
    ): Value = requireAssigned(name, env.lookup(name))
}

/** Ben's sequential rule: the plain evaluator, `b` initialized against the
 * outer `a`. => "16\n" */
public fun benRuleTranscript(): String = transcriptOn(::Evaluator, DEBATE)

/** Alyssa's 4.16 mechanism: the reservation rejects the early read.
 * => "Error: type mismatch: a is read before it is assigned\n" */
public fun alyssaRuleTranscript(): String = transcriptOn(::WithScanOut, DEBATE)

/** Eva's simultaneous rule: `b` sees the final `a` of 5. => "20\n" */
public fun evaRuleTranscript(): String = transcriptOn(::EvaSimultaneous, DEBATE)
