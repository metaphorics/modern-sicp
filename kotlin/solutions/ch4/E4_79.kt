// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.79: rule application with environments instead
// of renaming.

package sicp.ch4.solutions

import sicp.ch4.Frame
import sicp.ch4.QuerySystem
import sicp.ch4.conclusionOf
import sicp.ch4.ruleBodyOf
import sicp.ch4.singletonStream
import sicp.ch4.unifyMatch
import sicp.runtime.LStream
import sicp.runtime.Value

/**
 * Each rule application runs in a lexical layer of its own. Where the
 * section's engine tags every rule variable with a fresh global counter,
 * this engine tags it with the layer the application opens: the body's
 * references resolve to the innermost layer that owns the name, an
 * enclosing layer's variable stays reachable through the chain, and a
 * recursive application shadows its same-named ancestors only inside the
 * call. The layer discipline is the environment structure of a procedure
 * call; the data base, the index, and the stream combinators are the
 * substrate's, so answer order is the stream system's.
 */
public class ScopedQuerySystem : QuerySystem() {
    private var layerDepth = 0

    public override fun applyARule(
        rule: Value,
        queryPattern: Value,
        queryFrame: Frame,
    ): LStream<Frame> {
        // the layer is owned for the whole life of the body's answer
        // stream, so its id stays fixed while the lazy stream forces
        val layer = ++layerDepth
        val scoped = scopeTreeWalk(rule, layer)
        val unified = unifyMatch(queryPattern, conclusionOf(scoped), queryFrame) ?: return LStream.Empty
        return qeval(ruleBodyOf(scoped), singletonStream(unified))
    }

    private fun scopeTreeWalk(
        exp: Value,
        layer: Int,
    ): Value =
        when {
            isVar(exp) -> makeLayerVariable(exp, layer)
            exp is sicp.runtime.VPair -> cons(scopeTreeWalk(exp.car, layer), scopeTreeWalk(exp.cdr, layer))
            else -> exp
        }
}

private fun isVar(v: Value): Boolean = v is sicp.runtime.VTagged && v.tag == "?"

private fun cons(
    a: Value,
    b: Value,
): Value = sicp.runtime.cons(a, b)

/** `(? name)` in layer n becomes `(? n name)` -- the layer owns every
 * variable bound inside it, and the printed form is the book's. */
private fun makeLayerVariable(
    variable: Value,
    layer: Int,
): Value {
    val data = ((variable as sicp.runtime.VTagged).data as sicp.runtime.VSym).name
    return sicp.runtime.VTagged("?", sicp.runtime.VSym("$layer $data"))
}

/** The scoped engine matches the renaming engine answer for answer,
 * order included, on the recursive outranked-by rule -- including a
 * query whose bound variable flows into the rule through the chain. */
public fun scopedVersusRenaming(): List<String> {
    val out = mutableListOf<String>()
    val renaming = microshaftSystem()
    val scoped =
        ScopedQuerySystem().also {
            it.load(microshaftDatabase)
            it.load(proseRules)
        }
    for (
    query in
    listOf(
        "(outranked-by (Bitdiddle Ben) ?who)",
        "(outranked-by ?staff-person ?boss)",
        "(and (salary ?staff-person ?amount) (outranked-by ?staff-person ?boss))",
    )
    ) {
        val renameAnswers = answersOf(renaming, query)
        val scopedAnswers = answersOf(scoped, query)
        out.add("query: $query")
        out.add("renaming answers=${renameAnswers.size} scoped answers=${scopedAnswers.size} equal=${renameAnswers == scopedAnswers}")
        if (query == "(outranked-by (Bitdiddle Ben) ?who)") {
            out.addAll(scopedAnswers)
        }
    }
    return out
}
