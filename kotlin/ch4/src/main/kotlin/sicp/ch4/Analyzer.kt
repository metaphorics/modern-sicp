// SPDX-License-Identifier: GPL-3.0-only
// The analyzed evaluator of 4.1.7: `analyze` performs the syntactic
// analysis once per expression and returns the execution procedure, a
// `context(Raise<SchemeError>) (Env) -> Value` closure, that completes the
// evaluation. `Analyzer` is the same seam shape as `Evaluator`: `analyze`
// is the hook, an extension checks its forms and falls back to
// `baseAnalyze`, and the hook fires at every nesting depth because every
// analysis recurses through `analyze`.
//
// Compound procedures carry their analyzed body in an identity table keyed
// by the procedure object (the sibling editions' device): a hash over
// values is unstable because captured frames mutate, so lookup is by
// identity, and each analyzed `lambda` registers its body exactly once.

package sicp.ch4

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import sicp.runtime.AppE
import sicp.runtime.BeginE
import sicp.runtime.CondE
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.IfE
import sicp.runtime.LambdaE
import sicp.runtime.LetE
import sicp.runtime.LitE
import sicp.runtime.QuoteE
import sicp.runtime.SchemeError
import sicp.runtime.SetE
import sicp.runtime.VPrimitive
import sicp.runtime.VProc
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.VarE
import sicp.runtime.callPrimitive
import sicp.runtime.isTrue
import java.util.IdentityHashMap

/** The execution procedure: the analyzed expression awaiting an environment. */
public typealias Exec = context(Raise<SchemeError>)
(Env) -> Value

/**
 * The base analyzer. The book's clause chain lives in [analyze] and its
 * [baseAnalyze] fallback.
 */
public open class Analyzer(
    /** The global environment the driver set up. */
    public val global: Env,
) {
    /** Analyzed compound-procedure bodies, keyed by procedure identity. */
    private val bodies: MutableMap<VProc, Exec> = IdentityHashMap()

    /** Evaluates an already-parsed expression: `(analyze expr) env`. */
    context(r: Raise<SchemeError>)
    public fun eval(
        expr: Expr,
        env: Env,
    ): Value = analyze(expr)(env)

    /** The extension seam: check the forms you add, then fall back to
     * [baseAnalyze]. */
    public open fun analyze(expr: Expr): Exec = baseAnalyze(expr)

    /** The book's clause chain of 4.1.7. */
    protected fun baseAnalyze(expr: Expr): Exec =
        when (expr) {
            is LitE -> {
                analyzedSelfEvaluating(expr.v)
            }

            is QuoteE -> {
                analyzedQuoted(expr.datum)
            }

            is VarE -> {
                analyzedVariable(expr.name)
            }

            is SetE -> {
                analyzedAssignment(expr.name, analyze(expr.value))
            }

            is DefineE -> {
                analyzedDefinition(expr, expr.value as? LambdaE)
            }

            is IfE -> {
                analyzedIf(
                    analyze(expr.predicate),
                    analyze(expr.consequent),
                    analyze(expr.alternative),
                )
            }

            is LambdaE -> {
                analyzedLambda(expr.params, expr.rest, expr.body, null)
            }

            is BeginE -> {
                analyzeSequence(expr.actions)
            }

            is CondE -> {
                analyze(condToIfChain(expr.clauses))
            }

            // derived
            is LetE -> {
                analyze(letToCombinationExpr(expr))
            }

            // derived
            is AppE -> {
                analyzedApplication(expr)
            }
        }

    /** The book's `analyze-self-evaluating`. */
    protected fun analyzedSelfEvaluating(v: Value): Exec = { _ -> v }

    /** The book's `analyze-quoted`: the text extracted once, at analysis time. */
    protected fun analyzedQuoted(datum: Value): Exec = { _ -> datum }

    /** The book's `analyze-variable`: the lookup stays in the execution phase. */
    protected fun analyzedVariable(name: String): Exec = { env -> env.lookup(name) }

    /** The book's `analyze-assignment`. */
    protected fun analyzedAssignment(
        name: String,
        vproc: Exec,
    ): Exec =
        { env ->
            env.set(name, vproc(env))
            VSym("ok")
        }

    /** The book's `analyze-definition`; a procedure definition names its value. */
    protected fun analyzedDefinition(
        expr: DefineE,
        lambda: LambdaE?,
    ): Exec {
        val vproc: Exec =
            if (lambda != null) {
                analyzedLambda(lambda.params, lambda.rest, lambda.body, expr.name)
            } else {
                analyze(expr.value)
            }
        return { env ->
            env.define(expr.name, vproc(env))
            VSym("ok")
        }
    }

    /** The book's `analyze-if`. */
    protected fun analyzedIf(
        pproc: Exec,
        cproc: Exec,
        aproc: Exec,
    ): Exec =
        { env ->
            if (isTrue(pproc(env))) cproc(env) else aproc(env)
        }

    /** The book's `analyze-lambda`: the body analyzed once, registered under
     * the procedure object this execution procedure creates. */
    protected fun analyzedLambda(
        params: PersistentList<String>,
        rest: String?,
        body: PersistentList<Expr>,
        name: String?,
    ): Exec {
        val bproc = analyzeSequence(body)
        return { env ->
            val procedure = VProc(params, rest, body, env, name)
            bodies[procedure] = bproc
            procedure
        }
    }

    /** The book's `analyze-sequence`: the execution procedures composed at
     * analysis time, so the sequence itself is analyzed. Exercise 4.23
     * overrides this with Alyssa's looping version. */
    protected open fun analyzeSequence(actions: PersistentList<Expr>): Exec {
        if (actions.isEmpty()) {
            return { fail(SchemeError.TypeMismatch("Empty sequence: ANALYZE")) }
        }
        val procs = actions.map { analyze(it) }
        var combined: Exec = procs.last()
        for (i in procs.size - 2 downTo 0) {
            val earlier = procs[i]
            val later = combined
            combined = { env ->
                earlier(env)
                later(env)
            }
        }
        return combined
    }

    /** The book's `analyze-application`. */
    protected fun analyzedApplication(expr: AppE): Exec {
        val fproc = analyze(expr.operator)
        val aprocs = expr.operands.map { analyze(it) }
        return { env ->
            val procedure = fproc(env)
            val arguments = aprocs.map { it(env) }
            executeApplication(procedure, arguments)
        }
    }

    /** The book's `execute-application`: the compound body is already
     * analyzed, so the stored execution procedure runs on the extended
     * environment. */
    context(r: Raise<SchemeError>)
    protected open fun executeApplication(
        procedure: Value,
        arguments: List<Value>,
    ): Value =
        when (procedure) {
            is VPrimitive -> {
                callPrimitive(procedure, arguments)
            }

            is VProc -> {
                val body =
                    bodies[procedure]
                        ?: r.raise(SchemeError.MachineFault("unanalyzed procedure: $procedure"))
                val frame = Env.extend(procedure.params.toList(), arguments, procedure.env, procedure.rest)
                body(frame)
            }

            else -> {
                r.raise(SchemeError.NotApplicable(procedure))
            }
        }
}
