// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.73: the delay in flatten-stream keeps the
// recursive rest from being constructed with the first substream.

package sicp.ch4.solutions

import sicp.ch4.Frame
import sicp.ch4.QuerySystem
import sicp.ch4.flattenStream
import sicp.ch4.interleaveDelayed
import sicp.ch4.streamMap
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

/** The counterfactual: the rest of the substream list is constructed
 * eagerly, before the first element is demanded. */
public class EagerFlattenSystem : QuerySystem() {
    public override fun <T> flatmapFrames(
        proc: (T) -> LStream<Frame>,
        s: LStream<T>,
    ): LStream<Frame> = flattenEager(streamMap(proc, s))
}

private fun flattenEager(stream: LStream<LStream<Frame>>): LStream<Frame> {
    val first = stream as? LStream.Cons ?: return LStream.Empty
    val rest = flattenEager(first.tail)
    return interleaveDelayed(first.head) { rest }
}

private class Abandoned : RuntimeException()

/** An unbounded source of one-frame substreams; the counter is a budget
 * so a divergence is measured, not waited on. */
private fun substreamSource(counter: LongArray): LStream<LStream<Frame>> {
    counter[0] += 1
    if (counter[0] > 2000) throw Abandoned()
    return consStream(sicp.ch4.singletonStream(Frame.Empty)) { substreamSource(counter) }
}

/** On the infinite substream source the delayed combinator delivers
 * element after element with one procedure call each; the eager one is
 * still constructing before any element exists. Finite inputs agree
 * element for element, which is why the bug hides. */
public fun flattenDelayDebate(): List<String> {
    val out = mutableListOf<String>()
    val delayedCounter = LongArray(1)
    val delayed = flattenStream(substreamSource(delayedCounter))
    out.add("delayed_first3=${delayed.take(3).size} source_calls=${delayedCounter[0]}")
    val eagerCounter = LongArray(1)
    val eagerResult =
        try {
            val eager = flattenEager(substreamSource(eagerCounter))
            "first=${eager.take(1).size}"
        } catch (abandoned: Abandoned) {
            "diverged_before_first_answer=true"
        } catch (overflow: StackOverflowError) {
            "diverged_before_first_answer=true"
        }
    out.add("eager_$eagerResult source_calls=${eagerCounter[0]}")
    val finite = listOf(Frame.Empty, Frame.Empty, Frame.Empty)
    val a = flattenStream(sicp.ch4.listStream(listOf(sicp.ch4.listStream(finite), sicp.ch4.listStream(finite)))).take(6)
    val b = flattenEager(sicp.ch4.listStream(listOf(sicp.ch4.listStream(finite), sicp.ch4.listStream(finite)))).take(6)
    out.add("finite_orders_agree=${a == b}")
    return out
}
