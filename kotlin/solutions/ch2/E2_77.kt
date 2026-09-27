// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.77

package sicp.ch2.exercises

import arrow.core.raise.Raise
import arrow.core.raise.either
import java.util.concurrent.atomic.AtomicInteger

/**
 * Exercise 2.77: Louis Reasoner tries to evaluate `magnitude(z)` where `z`
 * is the two-level number of Figure 2.24 -- here `Complex(Rect(3.0, 4.0))`
 * -- and gets a `GenError.NoMethod` for `magnitude` on `(complex)`, because
 * the complex package installs `add`, `sub`, `mul`, and `div` but none of
 * the selectors. Alyssa's fix: install the four selectors at the complex
 * level, each forwarding to the representation level the outer tag was
 * hiding. Evaluating `magnitude` then dispatches twice -- once at the
 * complex level, once at the rectangular level -- which [ex_2_77] counts.
 *
 * Counts dispatches while the trace runs (test instrumentation).
 */
public class DispatchCounter {
    private val hits: AtomicInteger = AtomicInteger(0)

    /** Bumps and returns the new count. */
    public fun bump(): Int = hits.incrementAndGet()

    /** The count so far. */
    public fun count(): Int = hits.get()
}

/** Installs the rectangular and polar packages into `reps`, so the
 * representation level can answer forwarded selectors. */
public fun installRepresentationPackages(reps: RepTable) {
    reps.put("real-part", "rectangular") { z -> Real(repRealPart(z)) }
    reps.put("imag-part", "rectangular") { z -> Real(repImagPart(z)) }
    reps.put("magnitude", "rectangular") { z -> Real(repMagnitude(z)) }
    reps.put("angle", "rectangular") { z -> Real(repAngle(z)) }
    reps.put("real-part", "polar") { z -> Real(repRealPart(z)) }
    reps.put("imag-part", "polar") { z -> Real(repImagPart(z)) }
    reps.put("magnitude", "polar") { z -> Real(repMagnitude(z)) }
    reps.put("angle", "polar") { z -> Real(repAngle(z)) }
}

/** Alyssa's addition to the complex package: the four selectors,
 * installed at the complex level and forwarded through the representation
 * table. The counter records every representation-level dispatch the
 * forward performs. */
public fun installComplexSelectors(
    table: NumTable,
    reps: RepTable,
    counter: DispatchCounter,
) {
    fun forward(op: String): NumOp =
        { args ->
            val z = args.single()
            if (z !is Complex) raise(GenError.BadArgs(op, "expected a complex number"))
            countingApplyRepGeneric(counter, reps, op, z.rep)
        }
    table.put("real-part", listOf("complex"), forward("real-part"))
    table.put("imag-part", listOf("complex"), forward("imag-part"))
    table.put("magnitude", listOf("complex"), forward("magnitude"))
    table.put("angle", listOf("complex"), forward("angle"))
}

context(r: Raise<GenError>)
internal fun countingApplyGeneric(
    counter: DispatchCounter,
    table: NumTable,
    op: String,
    args: List<Num>,
): Num {
    counter.bump()
    return applyGeneric(table, op, args)
}

context(r: Raise<GenError>)
internal fun countingApplyRepGeneric(
    counter: DispatchCounter,
    reps: RepTable,
    op: String,
    rep: ComplexRep,
): Num {
    counter.bump()
    return applyRepGeneric(reps, op, rep)
}

/** Traces `magnitude` on the Figure 2.24 number and returns how many
 * times the dispatch ran: the answer to the exercise's "how many times"
 * question is 2. */
public fun ex_2_77(): Int {
    val counter = DispatchCounter()
    val table = NumTable()
    val reps = RepTable()
    installGenericArithmetic(table)
    installRepresentationPackages(reps)
    installComplexSelectors(table, reps, counter)
    val z = Complex(Rect(3.0, 4.0))
    val result = either { countingApplyGeneric(counter, table, "magnitude", listOf(z)) }
    check(result.getOrNull() == Real(5.0))
    return counter.count()
}
