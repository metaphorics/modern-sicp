// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, section 5.4: the measurement harness the stack exercises
// share (5.26 to 5.29). Every measurement runs the evaluator machine on
// the monitored driver of 5.4.4, reads the `(total-pushes = P
// maximum-depth = D)` lines the driver printed, and takes the call's own
// counters, so every number the exercises pin is machine-run output. The
// formula helpers fit their constants from the data and verify them on
// every measured point, so a formula line is a result of the run, not an
// assumption.

package sicp.ch5.solutions

import arrow.core.raise.either
import sicp.ch5.Op
import sicp.ch5.makeEvaluator
import sicp.ch5.monitoredEvaluatorController
import kotlin.math.abs

/** One interaction's monitored-stack counters. */
internal data class Stats(
    val pushes: Int,
    val depth: Int,
)

private val statsLine = Regex("""\(total-pushes = (\d+) maximum-depth = (\d+)\)""")

/** Runs [source] on the monitored driver and answers its transcript. */
internal fun runMonitored(source: String): List<String> = runMonitoredOn(monitoredEvaluatorController, source)

/** Runs [source] on [controller] with [operations] over the base table
 *  and answers its transcript; a failure is a measurement bug, so it
 *  fails fast. */
internal fun runMonitoredOn(
    controller: List<sicp.runtime.Stmt>,
    source: String,
    operations: Map<String, Op> = emptyMap(),
): List<String> =
    either {
        val evaluator = makeEvaluator(source, controller, operations)
        evaluator.drive()
        evaluator.transcript
    }.fold(
        { e -> error("the measurement failed: $e") },
        { it },
    )

/** The counters of every interaction that printed one, in order. */
internal fun statsOf(lines: List<String>): List<Stats> =
    lines.mapNotNull { line ->
        statsLine.matchEntire(line)?.destructured?.let { (pushes, depth) ->
            Stats(pushes.toInt(), depth.toInt())
        }
    }

/** Runs the program's calls, one per [ns], and answers the call's own
 *  counters each time (the last statistics line of the run). */
internal fun measure(
    source: String,
    ns: List<Int>,
    call: (Int) -> String,
): List<Stats> =
    ns.map { n ->
        val counters = statsOf(runMonitored("$source\n${call(n)}"))
        counters.lastOrNull() ?: error("the call printed no stack statistics")
    }

/** The `a`, `b` of `p(n) = an + b` through the first and last point;
 *  null for a single point. */
internal fun fitLinear(
    ns: List<Int>,
    ps: List<Int>,
): Pair<Int, Int>? {
    if (ns.size < 2) return null
    val slope = (ps.last() - ps.first()) / (ns.last() - ns.first())
    return slope to ps.first() - slope * ns.first()
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
    return "$what = ${a}n $sign ${abs(b)}, holds on every measured n: $holds"
}

/** Fits [ps] over [ns] and answers the formula line with the per-point
 *  verification; a single point is not linear. */
internal fun linearFitLine(
    what: String,
    ns: List<Int>,
    ps: List<Int>,
): String {
    val fit = fitLinear(ns, ps) ?: return "$what: not linear"
    val holds = ns.zip(ps).all { (n, p) -> fit.first * n + fit.second == p }
    return linearFormula(what, fit.first, fit.second, holds)
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

/** The iterative factorial of 1.2.1, 5.26's and 5.28's subject. */
internal val iterativeFactorialSource: String =
    """
    (define (factorial n)
      (define (iter product counter)
        (if (> counter n)
            product
            (iter (* counter product)
                  (+ counter 1))))
      (iter 1 1))
    """.trimIndent()

/** The recursive factorial, 5.27's and 5.28's subject. */
internal val recursiveFactorialSource: String =
    """
    (define (factorial n)
      (if (= n 1)
          1
          (* (factorial (- n 1)) n)))
    """.trimIndent()

/** The tree-recursive Fibonacci, 5.29's subject. */
internal val fibSource: String =
    """
    (define (fib n)
      (if (< n 2)
          n
          (+ (fib (- n 1)) (fib (- n 2)))))
    """.trimIndent()
