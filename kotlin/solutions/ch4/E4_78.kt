// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.78: the query language as a nondeterministic
// program on the amb evaluator of 4.3.

package sicp.ch4.solutions

import sicp.ch4.AmbEvaluator
import sicp.ch4.AmbExec
import sicp.ch4.AmbFail
import sicp.ch4.Frame
import sicp.ch4.QueryFault
import sicp.ch4.QuerySystem
import sicp.ch4.ambDriver
import sicp.ch4.conclusionOf
import sicp.ch4.contractQuestionMark
import sicp.ch4.listValueOf
import sicp.ch4.patternMatch
import sicp.ch4.querySyntaxProcess
import sicp.ch4.ruleBodyOf
import sicp.ch4.singletonStream
import sicp.ch4.unifyMatch
import sicp.runtime.AppE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.LStream
import sicp.runtime.LitE
import sicp.runtime.Random
import sicp.runtime.SchemeError
import sicp.runtime.VNil
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.VarE
import sicp.runtime.asSequence
import sicp.runtime.cons

/** Rebuilds the query datum the driver parsed, so the query forms can be
 * analyzed as data. */
private fun datum(expr: Expr): Value =
    when (expr) {
        is LitE -> expr.v
        is VarE -> VSym(expr.name)
        is AppE -> cons(datum(expr.operator), expr.operands.foldRight(VNil as Value) { e, acc -> cons(datum(e), acc) })
        else -> throw QueryFault(SchemeError.Parse("unsupported query form"))
    }

/**
 * Every enumeration point is a choice point of the amb engine: a simple
 * query chooses over its matching assertions and then its rule
 * applications, and an or chooses over its disjuncts -- depth-first,
 * which is exactly the behavioral difference from the interleaving
 * stream engine. The deterministic filters (not, lisp-value) re-enter
 * the substrate's stream machinery over the same data base, so no
 * choice point is spent on them.
 */
public class QueryAmbEvaluator(
    global: Env,
    random: Random?,
    public val database: QuerySystem,
) : AmbEvaluator(global, random) {
    private var frame: Frame = Frame.Empty

    public override fun analyze(expr: Expr): AmbExec {
        frame = Frame.Empty
        val query = querySyntaxProcess(datum(expr))
        val exec = analyzeQuery(query)
        return { env, succeed ->
            exec(env) { _ -> succeed(database.instantiate(query, frame) { v, _ -> VSym(contractQuestionMark(v)) }) }
        }
    }

    private fun analyzeQuery(query: Value): AmbExec {
        if (query !is sicp.runtime.VPair) return simpleQueryExec(query)
        val head = query.car as? VSym ?: return simpleQueryExec(query)
        return when (head.name) {
            "and" -> conjoinExec(listValueOf(query.cdr))
            "or" -> disjoinExec(listValueOf(query.cdr))
            "not", "lisp-value" -> filterExec(query)
            "always-true" -> succeedWith(VSym("ok"))
            else -> simpleQueryExec(query)
        }
    }

    private fun filterExec(query: Value): AmbExec =
        { _, succeed ->
            if (database.qeval(query, singletonStream(frame)) is LStream.Empty) throw AmbFail
            succeed(VSym("ok"))
        }

    private fun conjoinExec(conjuncts: List<Value>): AmbExec {
        if (conjuncts.isEmpty()) return { _, succeed -> succeed(VSym("ok")) }
        val first = analyzeQuery(conjuncts.first())
        val rest = conjoinExec(conjuncts.drop(1))
        return { env, succeed -> first(env) { _ -> rest(env, succeed) } }
    }

    private fun disjoinExec(disjuncts: List<Value>): AmbExec {
        if (disjuncts.isEmpty()) return { _, _ -> throw AmbFail }
        val alternatives: List<AmbExec> = disjuncts.map { analyzeQuery(it) }
        return { env, succeed ->
            val entryFrame = frame
            deliverChoice(alternatives.map { restartFrom(entryFrame, it) }, env, succeed)
        }
    }

    /** Each disjunct runs from the frame captured when the or is
     * delivered -- its true runtime entry frame. The wrapper restores
     * on every entry, so on choice-point re-entry after a sibling
     * disjunct exhausted, the shared var no longer holds the sibling's
     * last binding, which would filter this disjunct's matches away.
     * (`and` keeps the sequential flow of the shared var between
     * conjuncts.) */
    private fun restartFrom(
        entryFrame: Frame,
        exec: AmbExec,
    ): AmbExec =
        { env, succeed ->
            frame = entryFrame
            exec(env, succeed)
        }

    private fun simpleQueryExec(queryPattern: Value): AmbExec =
        { _, succeed ->
            val alternatives = mutableListOf<AmbExec>()
            for (assertion in database.fetchAssertions(queryPattern).asSequence().toList()) {
                val extended = patternMatch(queryPattern, assertion, frame)
                if (extended != null) {
                    val bound = extended
                    alternatives.add { _, succeed ->
                        frame = bound
                        succeed(VSym("ok"))
                    }
                }
            }
            for (rule in database.fetchRules(queryPattern).asSequence().toList()) {
                alternatives.add(ruleExec(rule, queryPattern))
            }
            deliverChoice(alternatives, global, succeed)
        }

    private fun ruleExec(
        rule: Value,
        queryPattern: Value,
    ): AmbExec =
        { _, succeed ->
            val clean = database.renameVariablesIn(rule)
            val unified = unifyMatch(queryPattern, conclusionOf(clean), frame) ?: throw AmbFail
            frame = unified
            analyzeQuery(ruleBodyOf(clean))(global, succeed)
        }
}

private fun queryAmb(assertions: String): sicp.ch4.AmbDriver {
    val shared = QuerySystem()
    shared.load(assertions)
    return ambDriver({ env, random -> QueryAmbEvaluator(env, random, shared) }, "")
}

/** The port answers one at a time through try-again and reports
 * exhaustion; the or is depth-first where the stream engine
 * interleaves; the recursive married rule still yields its cycle one
 * demandable answer at a time. */
public fun ambQueryDemos(): List<String> {
    val out = mutableListOf<String>()
    val jobs = queryAmb(microshaftDatabase)
    val jobSession = StringBuilder()
    jobSession.append(jobs.input("(job ?x (computer programmer))"))
    jobSession.append(jobs.input("try-again"))
    jobSession.append(jobs.input("try-again"))
    jobSession.append(jobs.input("try-again"))
    out.add("session: (job ?x (computer programmer))")
    out.add(jobSession.toString())
    val orAmb = queryAmb(microshaftDatabase)
    val orSession = StringBuilder()
    orSession.append(orAmb.input("(or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P)))"))
    repeat(4) { orSession.append(orAmb.input("try-again")) }
    out.add("session: (or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P))) -- amb is depth-first")
    out.add(orSession.toString())
    val streamOrder = answersOf(microshaftSystem(), "(or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P)))")
    out.add("the stream engine interleaves the same disjuncts: ${streamOrder.size} answers, first is ${streamOrder.firstOrNull()}")
    val married = queryAmb(microshaftDatabase + "\n(assert! (married Minnie Mickey))\n(assert! (rule (married ?x ?y)\n(married ?y ?x)))")
    val marriedSession = StringBuilder()
    marriedSession.append(married.input("(married Mickey ?who)"))
    marriedSession.append(married.input("try-again"))
    out.add("session: (married Mickey ?who) through the recursive rule")
    out.add(marriedSession.toString())
    return out
}
