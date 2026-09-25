// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.76: and as a merge of compatible frames.

package sicp.ch4.solutions

import sicp.ch4.Frame
import sicp.ch4.QuerySystem
import sicp.ch4.bindingInFrame
import sicp.ch4.firstConjunct
import sicp.ch4.isEmptyConjunction
import sicp.ch4.listStream
import sicp.ch4.restConjuncts
import sicp.ch4.singletonStream
import sicp.ch4.unifyMatch
import sicp.runtime.LStream
import sicp.runtime.Value
import sicp.runtime.take

/** The merging procedure the exercise asks for: each binding of the
 * second frame joins the first. A variable unbound in the first simply
 * extends it; a bound variable must unify with the proposed value, and
 * a dead unify rejects the pair. Every binding examination counts one
 * compatibility check. */
private fun MergeAndSystem.mergeFrames(
    first: Frame,
    second: Frame,
): Frame? {
    var result = first
    for ((variable, value) in second.bindings) {
        compatibilityChecks += 1
        val current = bindingInFrame(variable, result)
        result =
            if (current == null) {
                result.extended(variable, value)
            } else {
                unifyMatch(current, value, result) ?: return null
            }
    }
    return result
}

/** The merge-and special form: both conjuncts run separately over the
 * input, then compatible frame pairs merge. */
public class MergeAndSystem : QuerySystem() {
    public var compatibilityChecks: Long = 0L

    public override fun installDispatch() {
        super.installDispatch()
        putQuery("merge-and", ::mergeConjoin)
    }

    private fun mergeConjoin(
        conjuncts: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> {
        if (isEmptyConjunction(conjuncts)) return frameStream
        val first = qeval(firstConjunct(conjuncts), frameStream)
        val rest = mergeConjoin(restConjuncts(conjuncts), singletonStream(Frame.Empty))
        return flatmapFrames(
            { left ->
                listStream(rest.take(1000).mapNotNull { right -> mergeFrames(left, right) })
            },
            first,
        )
    }
}

public fun mergeAndSystem(): MergeAndSystem {
    val system = MergeAndSystem()
    system.load(microshaftDatabase)
    system.load(proseRules)
    return system
}

/** Merge-and matches the series and on two shared queries; the check
 * counts are the price comparison the exercise sketches. */
public fun mergeAndDemos(): List<String> {
    val out = mutableListOf<String>()
    val series = microshaftSystem()
    val merge = mergeAndSystem()
    for (
    pair in
    listOf(
        "(merge-and (job ?x (computer programmer)) (supervisor ?x ?boss))" to
            "(and (job ?x (computer programmer)) (supervisor ?x ?boss))",
        "(merge-and (supervisor ?x ?y) (job ?x ?job))" to
            "(and (supervisor ?x ?y) (job ?x ?job))",
    )
    ) {
        val (mergeQuery, seriesQuery) = pair
        // the merge answers name their head merge-and; rename it back so
        // the two listings compare
        val mergeAnswers =
            answersOf(merge, mergeQuery).map { it.replaceFirst("(merge-and ", "(and ") }
        val seriesAnswers = answersOf(series, seriesQuery)
        out.add("query: $mergeQuery")
        out.add(
            "answers=${mergeAnswers.size} same_answers_as_series: " +
                "${mergeAnswers.sorted() == seriesAnswers.sorted()} compatibility checks: ${merge.compatibilityChecks}",
        )
        merge.compatibilityChecks = 0
    }
    return out
}
