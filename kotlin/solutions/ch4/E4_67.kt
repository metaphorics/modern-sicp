// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.67: the loop detector -- a history of deduction
// keys over (query pattern, frame), cutting chains that revisit a key.

package sicp.ch4.solutions

import sicp.ch4.Frame
import sicp.ch4.QuerySystem
import sicp.ch4.contractQuestionMark
import sicp.ch4.printValue
import sicp.runtime.LStream
import sicp.runtime.VSym
import sicp.runtime.Value

private const val MARRIED =
    """
    (assert! (married Minnie Mickey))
    (assert! (rule (married ?x ?y)
                   (married ?y ?x)))
    """

/** Keeps one history per system: a rule application whose fully
 * resolved query pattern was already pursued along this chain is cut.
 * Resolution follows the frame's variable chains, so a cycle closes on
 * itself and the cut is exact -- the same pattern over a genuinely new
 * frame still passes. */
public class LoopCheckingSystem : QuerySystem() {
    private val history = HashSet<String>()
    public var chainsCut: Int = 0
        private set

    public override fun applyARule(
        rule: Value,
        queryPattern: Value,
        queryFrame: Frame,
    ): LStream<Frame> {
        // the key instantiates the pattern against the frame and
        // wildcards whatever is still unbound, so equivalent deduction
        // states collapse no matter which application level named them
        val resolved = instantiate(queryPattern, queryFrame) { _, _ -> VSym("*") }
        val key = printValue(resolved)
        if (!history.add(key)) {
            chainsCut += 1
            return LStream.Empty
        }
        return super.applyARule(rule, queryPattern, queryFrame)
    }
}

public fun loopCheckingSystem(): LoopCheckingSystem {
    val system = LoopCheckingSystem()
    system.load(microshaftDatabase)
    system.load(proseRules)
    system.load(MARRIED)
    return system
}

/** The married cycle terminates with the answer delivered once; the
 * wheel and the anchored outranked-by keep their stock answers because
 * no chain revisits a key. */
private fun bounded(block: () -> List<String>): List<String> =
    try {
        block()
    } catch (overflow: StackOverflowError) {
        listOf("the unanchored generation overflows this engine before returning")
    }

public fun loopDetectorDemos(): List<String> {
    val out = mutableListOf<String>()
    val system = loopCheckingSystem()
    val marriedAnswers = bounded { answersOf(system, "(married Mickey ?who)") }
    out.add("married Mickey ?who under the detector: ${marriedAnswers.size} answer(s), ${system.chainsCut} chain(s) cut, terminates")
    out.addAll(marriedAnswers)
    val wheel = loopCheckingSystem()
    val detected = answersOf(wheel, "(wheel ?who)")
    val stock = answersOf(microshaftSystem(), "(wheel ?who)")
    out.add("wheel under the detector: ${detected.size} answers, identical to stock: ${detected == stock}")
    val outranked = loopCheckingSystem()
    val outrankedAnswers = answersOf(outranked, "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))")
    val outrankedStock = answersOf(microshaftSystem(), "(outranked-by (Bitdiddle Ben) (Warbucks Oliver))")
    out.add(
        "outranked-by under the detector: ${outrankedAnswers.size} answer(s), identical to stock: ${outrankedAnswers == outrankedStock}",
    )
    return out
}
