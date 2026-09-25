// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.72: interleaving keeps every disjunct alive
// where appending starves the later ones.

package sicp.ch4.solutions

import sicp.ch4.Frame
import sicp.ch4.QuerySystem
import sicp.ch4.firstDisjunct
import sicp.ch4.isEmptyDisjunction
import sicp.ch4.restDisjuncts
import sicp.runtime.LStream
import sicp.runtime.Value

private const val LOVES =
    """
    (assert! (loves (Minnie Mouse) (Mickey Mouse)))
    (assert! (rule (loves ?x ?y)
                   (loves ?y ?x)))
    """

/** The counterfactual: disjoin by plain stream-append. */
public class AppendDisjoinSystem : QuerySystem() {
    public override fun disjoin(
        disjuncts: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> {
        if (isEmptyDisjunction(disjuncts)) return LStream.Empty
        return sicp.ch4.streamAppend(qeval(firstDisjunct(disjuncts), frameStream)) {
            disjoin(restDisjuncts(disjuncts), frameStream)
        }
    }
}

public fun lovesSystems(): Pair<QuerySystem, QuerySystem> {
    val interleave = microshaftSystem()
    interleave.load(LOVES)
    val append = microshaftSystem()
    append.load(LOVES)
    return Pair(
        interleave,
        AppendDisjoinSystem().also {
            it.load(microshaftDatabase)
            it.load(proseRules)
            it.load(LOVES)
        },
    )
}

private fun supervisorCount(answers: List<String>): Int = answers.count { it.contains("supervisor (") }

/** First eight answers under each combinator, and how many of them come
 * from the finite disjunct. */
public fun interleaveVersusAppend(): List<String> {
    val (interleave, append) = lovesSystems()
    val out = mutableListOf<String>()
    val query = "(or (loves ?a ?b) (supervisor ?who (Bitdiddle Ben)))"
    val first8 = answersUpto(interleave, query, 8)
    out.add("interleave_first8")
    out.addAll(first8)
    out.add("interleave_supervisor_answers_in_first8=${supervisorCount(first8)}")
    val append8 = answersUpto(append, query, 8)
    out.add("append_first8")
    out.addAll(append8)
    out.add("append_supervisor_answers_in_first8=${supervisorCount(append8)}")
    val append20 = answersUpto(append, query, 20)
    out.add("append_supervisor_answers_in_first20=${supervisorCount(append20)}")
    return out
}
