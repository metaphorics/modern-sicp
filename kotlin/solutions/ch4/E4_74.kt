// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.74: Alyssa's simple-stream-flatmap.

package sicp.ch4.solutions

import sicp.ch4.Frame
import sicp.ch4.QuerySystem
import sicp.ch4.negatedQuery
import sicp.ch4.singletonStream
import sicp.ch4.streamFilter
import sicp.ch4.streamMap
import sicp.runtime.LStream
import sicp.runtime.Value

/** (a) The missing expressions: keep the nonempty substreams and take
 * each one's only frame. */
public fun <T> simpleStreamFlatmap(
    proc: (T) -> LStream<Frame>,
    s: LStream<T>,
): LStream<Frame> = streamMap({ sub -> (sub as LStream.Cons).head }, streamFilter({ it is LStream.Cons }, streamMap(proc, s)))

/** Alyssa's combinator installed in exactly the three procedures she
 * names: negate, lisp-value, and find-assertions; simple-query keeps the
 * interleaving flatmap because its substreams are not singletons. */
public class SimpleFlatmapSystem : QuerySystem() {
    public var inputFrames: Long = 0L

    private fun <T> counted(
        proc: (T) -> LStream<Frame>,
        s: LStream<T>,
    ): LStream<Frame> =
        simpleStreamFlatmap(
            { item ->
                inputFrames += 1
                proc(item)
            },
            s,
        )

    public override fun negate(
        operands: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> =
        counted({ frame ->
            if (qeval(negatedQuery(operands), singletonStream(frame)) is LStream.Empty) {
                singletonStream(frame)
            } else {
                LStream.Empty
            }
        }, frameStream)

    public override fun lispValue(
        operands: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> =
        counted({ frame ->
            val call = instantiateCall(operands, frame)
            if (executeCall(call)) singletonStream(frame) else LStream.Empty
        }, frameStream)

    public override fun findAssertions(
        pattern: Value,
        frame: Frame,
    ): LStream<Frame> = counted({ checkAnAssertion(it, pattern, frame) }, fetchAssertions(pattern))
}

public fun simpleFlatmapSystem(): SimpleFlatmapSystem {
    val system = SimpleFlatmapSystem()
    system.load(microshaftDatabase)
    system.load(proseRules)
    return system
}

/** The answers are the same as the book's flatmap gives, because each
 * procedure mapped here produces at most one frame per input, so there
 * is nothing to interleave. */
public fun simpleFlatmapDemo(): List<String> {
    val out = mutableListOf<String>()
    val stock = microshaftSystem()
    val simple = simpleFlatmapSystem()
    val notQuery = "(and (supervisor ?x ?y) (not (job ?x (computer programmer))))"
    val stockAnswers = answersOf(stock, notQuery)
    val simpleAnswers = answersOf(simple, notQuery)
    out.add("not_query_answers=${simpleAnswers.size} answers_equal=${stockAnswers == simpleAnswers} input_frames=${simple.inputFrames}")
    simple.inputFrames = 0
    val lispQuery = "(and (salary ?person ?amount) (lisp-value > ?amount 30000))"
    val lispStock = answersOf(stock, lispQuery)
    val lispSimple = answersOf(simple, lispQuery)
    out.add("lisp_query_answers=${lispSimple.size} answers_equal=${lispStock == lispSimple} input_frames=${simple.inputFrames}")
    return out
}
