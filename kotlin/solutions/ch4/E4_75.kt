// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.75: the unique special form, installed through
// the qeval table.

package sicp.ch4.solutions

import sicp.ch4.Frame
import sicp.ch4.QuerySystem
import sicp.ch4.singletonStream
import sicp.runtime.LStream
import sicp.runtime.Value
import sicp.runtime.take

/** The qeval handler: keep only the frames whose unique query has
 * exactly one extension -- the shape of not, with a length test. */
public class UniqueSystem : QuerySystem() {
    public override fun installDispatch() {
        super.installDispatch()
        putQuery("unique", ::uniquelyAsserted)
    }

    private fun uniquelyAsserted(
        operands: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> {
        // the dispatched handler receives the form's contents, whose car
        // is the query whose extensions must number exactly one
        val query = (operands as sicp.runtime.VPair).car
        return flatmapFrames(
            { frame ->
                if (qeval(query, singletonStream(frame)).take(2).size == 1) {
                    singletonStream(frame)
                } else {
                    LStream.Empty
                }
            },
            frameStream,
        )
    }
}

public fun uniqueSystem(): UniqueSystem {
    val system = UniqueSystem()
    system.load(microshaftDatabase)
    system.load(proseRules)
    return system
}

/** The book's two singleton tests, the filled-by-one-person query, and
 * the supervise-precisely-one-person query. */
public fun uniqueDemos(): List<String> {
    val out = mutableListOf<String>()
    val system = uniqueSystem()
    out.add("query: (unique (job ?x (computer wizard)))")
    out.addAll(answersOf(system, "(unique (job ?x (computer wizard)))"))
    out.add("query: (unique (job ?x (computer programmer)))")
    out.addAll(answersOf(system, "(unique (job ?x (computer programmer)))"))
    out.add("query: (and (job ?x ?j) (unique (job ?anyone ?j)))")
    val filled = answersOf(system, "(and (job ?x ?j) (unique (job ?anyone ?j)))")
    out.add("answers=${filled.size}")
    out.addAll(filled)
    val singleton = answersOf(system, "(and (supervisor ?person ?boss) (unique (supervisor ?underling ?boss)))")
    out.add("query: (and (supervisor ?person ?boss) (unique (supervisor ?underling ?boss))) -- answers=${singleton.size}")
    out.addAll(singleton)
    return out
}
