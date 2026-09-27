// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.71: Louis Reasoner's undelayed simple-query and
// disjoin, which construct the whole recursion before any answer exists.

package sicp.ch4.solutions

import sicp.ch4.Frame
import sicp.ch4.QuerySystem
import sicp.ch4.firstDisjunct
import sicp.ch4.isEmptyDisjunction
import sicp.ch4.restDisjuncts
import sicp.runtime.LStream
import sicp.runtime.Value

/** The book's simpler definitions: the rule side of a simple query is
 * constructed with the assertion side, and disjoin constructs its
 * recursive rest before interleaving. */
public class LouisSystem : QuerySystem() {
    public var ruleApplications: Long = 0L

    public override fun applyARule(
        rule: Value,
        queryPattern: Value,
        queryFrame: Frame,
    ): LStream<Frame> {
        ruleApplications += 1
        return super.applyARule(rule, queryPattern, queryFrame)
    }

    public override fun simpleQuery(
        queryPattern: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> =
        flatmapFrames(
            { frame ->
                val rules = applyRules(queryPattern, frame)
                sicp.ch4.streamAppend(findAssertions(queryPattern, frame)) { rules }
            },
            frameStream,
        )

    public override fun disjoin(
        disjuncts: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> {
        if (isEmptyDisjunction(disjuncts)) return LStream.Empty
        val rest = disjoin(restDisjuncts(disjuncts), frameStream)
        return sicp.ch4.interleave(qeval(firstDisjunct(disjuncts), frameStream)) { rest }
    }
}

public fun undelayedSystem(): LouisSystem {
    val system = LouisSystem()
    system.load(microshaftDatabase)
    system.load(proseRules)
    return system
}

private const val MARRIED =
    """
    (assert! (married Minnie Mickey))
    (assert! (rule (married ?x ?y)
                   (married ?y ?x)))
    """

public fun marriedSystem(make: () -> QuerySystem): QuerySystem {
    val system = make()
    system.load(microshaftDatabase)
    system.load(MARRIED)
    return system
}

/** The delayed engine streams the supervisor-cycle answer on demand;
 * Louis's engine is still constructing it after the budget runs out. */
public fun delayDebate(): List<String> {
    val out = mutableListOf<String>()
    val stock = microshaftSystem()
    out.add("delayed engine, first three answers of the unanchored query:")
    out.addAll(answersUpto(stock, "(outranked-by ?staff-person ?boss)", 3))
    val louis = undelayedSystem()
    val before = louis.ruleApplications
    val first =
        try {
            answersUpto(louis, "(outranked-by ?staff-person ?boss)", 1).size
        } catch (fault: StackOverflowError) {
            -1
        }
    val after = louis.ruleApplications
    out.add("louis (plain stream-append in simple_query, plain interleave in disjoin): $first answer(s)")
    out.add(
        "constructing the first answer reached $after rule applications -- ${if (after > 1001) "still applying rules past 1001: the construction itself diverges" else "returned"}",
    )
    val marriedDelayed = marriedSystem { microshaftSystem() }
    out.add("married cycle, delayed engine, first three answers:")
    out.addAll(answersUpto(marriedDelayed, "(married Mickey ?who)", 3))
    val marriedLouis = marriedSystem { undelayedSystem() }
    val marriedResult =
        try {
            answersUpto(marriedLouis, "(married Mickey ?who)", 1).size
        } catch (fault: StackOverflowError) {
            -1
        }
    out.add(
        "married cycle under louis: $marriedResult answer(s) -- ${if (marriedResult < 0) "the construction diverges before any answer (engine recursion exhausted)" else "returned"}",
    )
    return out
}
