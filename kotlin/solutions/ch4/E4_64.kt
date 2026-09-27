// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.64: Louis Reasoner's swapped outranked-by, whose
// recursion precedes the supervisor test and diverges.

package sicp.ch4.solutions

import sicp.ch4.Frame
import sicp.ch4.QueryFault
import sicp.ch4.QuerySystem
import sicp.runtime.LStream
import sicp.runtime.Value

private const val LOUIS_RULE =
    """
    (assert! (rule (outranked-by ?staff-person ?boss)
                   (or (supervisor ?staff-person ?boss)
                       (and (outranked-by ?middle-manager ?boss)
                            (supervisor ?staff-person ?middle-manager)))))
    """

/** Counts rule applications so a divergence is measurable as a budget
 * overrun instead of a hang. */
public class CountingSystem : QuerySystem() {
    public var ruleApplications: Long = 0L

    public override fun applyARule(
        rule: Value,
        queryPattern: Value,
        queryFrame: Frame,
    ): LStream<Frame> {
        ruleApplications += 1
        return super.applyARule(rule, queryPattern, queryFrame)
    }
}

public fun louisSystem(): CountingSystem {
    val system = CountingSystem()
    system.load(microshaftDatabase)
    system.load(LOUIS_RULE)
    return system
}

/** The anchored query delivers its one answer, but forcing a second one
 * diverges: the recursion re-enumerates every level forever and the
 * supervisor test can never fail the recursed frame. The book's
 * conjunct order answers the same query completely. */
public fun louisOutranked(): List<String> {
    val louis = louisSystem()
    val out = mutableListOf<String>()
    out.add("(outranked-by (Bitdiddle Ben) (Warbucks Oliver)) under Louis's swapped rule:")
    val first =
        try {
            answersUpto(louis, "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))", 1).size
        } catch (overflow: StackOverflowError) {
            -1
        }
    out.add(
        "first answer: $first -- " +
            if (first < 0) {
                "the recursion-first and re-enumerates the whole closure at construction, before any answer exists"
            } else {
                "delivered"
            },
    )
    val stock = microshaftSystem()
    out.add("the book's conjunct order answers completely:")
    out.addAll(answersOf(stock, "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"))
    return out
}
