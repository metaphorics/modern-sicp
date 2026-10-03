// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, section 5.4: the measurement harness the stack exercises
// share (5.26 to 5.29). Every measurement admits one checked program
// whose `main` performs exactly the measured call, runs it on the
// explicit-control machine under a step budget, and reads that run's
// monitored stack counters, so every number the exercises pin is
// machine-run output. The formula helpers fit their constants from the
// data and verify them on every measured point, so a formula line is a
// result of the run, not an assumption.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.ExplicitControl
import sicp.guest.Admission
import sicp.guest.GuestError
import sicp.guest.Mode
import sicp.runtime.Machine

/** One measured run's monitored-stack counters. */
internal data class Stats(
    val pushes: Int,
    val depth: Int,
)

/** The step budget of one measured run: a machine that does not halt is
 *  a measurement bug, not a hang. */
private const val MEASURED_STEPS: Long = 50_000_000

/** Runs [source] to its halt on the explicit-control machine and answers
 *  the machine, whose stack counters the exercises read. */
internal fun monitoredMachine(source: String): Machine {
    val checked =
        Admission.admit(source, Mode.CORE).fold(
            { error -> error("the measurement source did not admit: $error") },
            { it },
        )
    val machine = ExplicitControl.machine(checked)
    var steps = 0L
    val outcome =
        either<GuestError, Unit> {
            while (!machine.halted() && steps < MEASURED_STEPS) {
                machine.step()
                steps++
            }
        }
    outcome.fold({ error -> error("the evaluator machine faulted: ${error.category}") }, { })
    if (!machine.halted()) error("the evaluator machine did not halt within $MEASURED_STEPS steps")
    return machine
}

/** The monitored stack of one run of [source]. */
internal fun measureOne(source: String): Stats {
    val machine = monitoredMachine(source)
    return Stats(machine.stack.pushes.toInt(), machine.stack.maxDepth.toInt())
}

/** The `a`, `b` of `p(n) = an + b` through the first and last point;
 *  null for a single point. */
internal fun fitLinear(
    ns: List<Int>,
    ps: List<Int>,
): Pair<Int, Int>? {
    if (ns.size < 2) return null
    val first = ns.first()
    val last = ns.last()
    val rise = ps.last() - ps.first()
    val run = last - first
    if (run == 0 || rise % run != 0) return null
    val a = rise / run
    val b = ps.first() - a * first
    return a to b
}

/** One table row the way the exercises print it. */
internal fun renderStats(
    name: String,
    n: Int,
    s: Stats,
): String = "$name n=$n: total-pushes = ${s.pushes} maximum-depth = ${s.depth}"

/** A fitted linear formula line, with the verification against every
 *  measured point stated on the line. */
internal fun linearFormula(
    what: String,
    a: Int,
    b: Int,
    holds: Boolean,
): String {
    val sign = if (b < 0) "-" else "+"
    return "$what = ${a}n $sign ${kotlin.math.abs(b)}, holds on every measured n: $holds"
}

/** Whether `p(n) = an + b` holds on every measured point. */
internal fun holdsLinear(
    ns: List<Int>,
    ps: List<Int>,
    a: Int,
    b: Int,
): Boolean = ns.zip(ps).all { (n, p) -> a * n + b == p }

/** Fits [ps] over [ns] and answers the formula line with the per-point
 *  verification; a single point is not linear. */
internal fun linearFitLine(
    what: String,
    ns: List<Int>,
    ps: List<Int>,
): String {
    val fit = fitLinear(ns, ps) ?: return "$what: not linear on a single point"
    return linearFormula(what, fit.first, fit.second, holdsLinear(ns, ps, fit.first, fit.second))
}

/** The n-th Fibonacci number, the oracle for 5.29's formula check. */
internal fun fib(n: Int): Long {
    var a = 0L
    var b = 1L
    repeat(n) {
        val next = a + b
        a = b
        b = next
    }
    return a
}

/** The iterative factorial of 1.2.1, 5.26's and 5.28's subject: the
 *  internal `iter` closes over `n`, as the book's does. */
internal val iterativeFactorialSource: String =
    """
    fun factorial(n: Long): Long {
        fun iter(product: Long, counter: Long): Long {
            if (counter > n) {
                return product
            }
            return iter(counter * product, counter + 1L)
        }
        return iter(1L, 1L)
    }
    """.trimIndent()

/** The recursive factorial, 5.27's and 5.28's subject. */
internal val recursiveFactorialSource: String =
    """
    fun factorial(n: Long): Long {
        if (n == 1L) {
            return 1L
        }
        return factorial(n - 1L) * n
    }
    """.trimIndent()

/** The tree-recursive Fibonacci, 5.29's subject. */
internal val fibSource: String =
    """
    fun fib(n: Long): Long {
        if (n < 2L) {
            return n
        }
        return fib(n - 1L) + fib(n - 2L)
    }
    """.trimIndent()

/** The measured call of [body]: a program whose `main` performs exactly
 *  [body], so the run's counters are the call's own. `main` keeps the
 *  admitted shape -- no declared return type, block body -- and the
 *  discarded result does not affect the counters being measured. */
internal fun measuredCall(
    definitions: String,
    body: String,
): String = definitions + "\nfun main() {\n    $body\n}\n"
