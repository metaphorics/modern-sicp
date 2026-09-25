// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.77: delayed not and lisp-value filters -- a
// filter whose variables are unbound becomes a promise carried on the
// frame and is fulfilled as soon as the bindings exist.

package sicp.ch4.solutions

import sicp.ch4.Frame
import sicp.ch4.QuerySystem
import sicp.ch4.bindingInFrame
import sicp.ch4.firstConjunct
import sicp.ch4.isEmptyConjunction
import sicp.ch4.isVar
import sicp.ch4.negatedQuery
import sicp.ch4.restConjuncts
import sicp.ch4.singletonStream
import sicp.runtime.LStream
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.VTagged
import sicp.runtime.Value

/** The promise: a frame binding whose key is a fresh deferred marker and
 * whose value is the filter's query. */
public class DeferredFilterSystem : QuerySystem() {
    private var markerId = 0
    public var deferredCreated: Long = 0L
    public var fulfilled: Long = 0L
    public var unresolved: Long = 0L

    private fun variablesOf(
        exp: Value,
        into: MutableSet<Value>,
    ) {
        when {
            isVar(exp) -> {
                into.add(exp)
            }

            exp is VPair -> {
                variablesOf(exp.car, into)
                variablesOf(exp.cdr, into)
            }

            else -> {}
        }
    }

    private fun ready(
        query: Value,
        frame: Frame,
    ): Boolean {
        val vars = mutableSetOf<Value>()
        variablesOf(query, vars)
        return vars.all { bindingInFrame(it, frame) != null }
    }

    private fun markerKey(kind: String): Value {
        markerId += 1
        return VTagged(kind, VSym("d-$markerId"))
    }

    /** not over an unbound variable defers; over bound variables it is
     * the stock filter. */
    public override fun negate(
        operands: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> =
        flatmapFrames(
            { frame ->
                val query = negatedQuery(operands)
                if (ready(query, frame)) {
                    if (qeval(query, singletonStream(frame)) is LStream.Empty) {
                        singletonStream(frame)
                    } else {
                        LStream.Empty
                    }
                } else {
                    deferredCreated += 1
                    singletonStream(frame.extended(markerKey("deferred-not"), query))
                }
            },
            frameStream,
        )

    /** lisp-value defers the same way; the full form re-enters the
     * underlying filter once its variables exist. */
    public override fun lispValue(
        operands: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> {
        val fullForm = VPair(VSym("lisp-value"), operands)
        return flatmapFrames(
            { frame ->
                if (ready(fullForm, frame)) {
                    if (qeval(fullForm, singletonStream(frame)) is LStream.Empty) LStream.Empty else singletonStream(frame)
                } else {
                    deferredCreated += 1
                    singletonStream(frame.extended(markerKey("deferred-lisp"), fullForm))
                }
            },
            frameStream,
        )
    }

    /** Fulfillment: run every promise whose variables are now bound; a
     * fulfilled filter that fails kills the frame. */
    private fun fulfill(frame: Frame): Frame? {
        var result = frame
        var progressed = true
        while (progressed) {
            progressed = false
            for ((key, query) in result.bindings.entries.toList()) {
                if (key !is VTagged || (key.tag != "deferred-not" && key.tag != "deferred-lisp")) continue
                if (!ready(query, result)) continue
                progressed = true
                fulfilled += 1
                val stripped = Frame(result.bindings.removing(key))
                val satisfied = qeval(query, singletonStream(stripped)) !is LStream.Empty
                // a not marker passes when the subquery fails; a
                // lisp-value marker passes when it succeeds
                val passes = if (key.tag == "deferred-not") !satisfied else satisfied
                if (!passes) return null
                result = stripped
            }
        }
        return result
    }

    public override fun conjoin(
        conjuncts: Value,
        frameStream: LStream<Frame>,
    ): LStream<Frame> {
        if (isEmptyConjunction(conjuncts)) {
            return flatmapFrames({ f -> fulfill(f)?.let { singletonStream(it) } ?: LStream.Empty }, frameStream)
        }
        val next =
            flatmapFrames({ f -> fulfill(f)?.let { singletonStream(it) } ?: LStream.Empty }, qeval(firstConjunct(conjuncts), frameStream))
        return conjoin(restConjuncts(conjuncts), next)
    }

    /** Any promise left on a finished frame stays unresolved. */
    public fun countUnresolved(frame: Frame) {
        val pending = frame.bindings.keys.count { it is VTagged && it.tag == "deferred" }
        unresolved += pending
    }
}

public fun deferredSystem(): DeferredFilterSystem {
    val system = DeferredFilterSystem()
    system.load(microshaftDatabase)
    system.load(proseRules)
    return system
}

/** The two wrong-answer cases of 4.4.3 with the filters written first,
 * plus the already-bound order for contrast. */
private fun boundedQuery(
    system: DeferredFilterSystem,
    query: String,
): List<String> =
    try {
        answersOf(system, query)
    } catch (overflow: StackOverflowError) {
        listOf("the naive order diverges in this engine before answering")
    }

public fun delayedFilterDemos(): List<String> {
    val out = mutableListOf<String>()
    val system = deferredSystem()
    val notFirst = "(and (not (job ?x (computer programmer))) (supervisor ?x ?y))"
    out.add("query: $notFirst")
    out.addAll(boundedQuery(system, notFirst))
    out.add("deferred=${system.deferredCreated} fulfilled=${system.fulfilled} unresolved=${system.unresolved}")
    val lispFirst = "(and (lisp-value > ?amount 30000) (salary ?who ?amount))"
    out.add("query: $lispFirst")
    out.addAll(boundedQuery(system, lispFirst))
    out.add("deferred=${system.deferredCreated} fulfilled=${system.fulfilled} unresolved=${system.unresolved}")
    val boundOrder = "(and (salary ?who ?amount) (lisp-value > ?amount 30000))"
    val boundAnswers = boundedQuery(system, boundOrder)
    out.add("query: $boundOrder -- answers=${boundAnswers.size}")
    out.addAll(boundAnswers)
    out.add("deferred=${system.deferredCreated} fulfilled=${system.fulfilled} unresolved=${system.unresolved}")
    return out
}
