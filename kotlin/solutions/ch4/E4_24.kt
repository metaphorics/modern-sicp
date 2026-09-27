// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.24

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.Analyzer
import sicp.ch4.Evaluator
import sicp.ch4.OutputSink
import sicp.ch4.formatError
import sicp.ch4.parseExpr
import sicp.ch4.readDatum
import sicp.ch4.setupEnvironment
import sicp.runtime.Value
import kotlin.system.measureNanoTime

// Exercise 4.24: the direct evaluator against the analyzer on tree
// recursion. Both engines run the same `(fib 12)` program in a fresh
// global environment; each reading is the median of [SAMPLES] timed
// batches of [TIMED_CALLS] calls, taken after [WARMUP_CALLS] unmeasured
// calls. The TEST asserts only structural facts -- both engines compute
// the same value and both medians are positive -- because a timing
// assertion would flake on any other host. The measured ratio goes in the
// `// =>` comment and the rationale, reported honestly for one run on one
// machine.
//
// Measured on the recording run (Xeon Gold 6138, Kotlin/JVM): medians of
// 7 batches of 100 calls of `(fib 12)` -- direct 43.3 ms, analyzer
// 53.9 ms, ratio direct/analyzer = 0.80, i.e. the ANALYZER ran about
// 1.24x slower here, and repeat runs kept that direction (direct
// 40.9-45.1 ms, analyzer 47.4-53.9 ms, ratio 0.80-0.94 over five runs).
// The library's direct evaluator is
// already a compiled dispatch loop with tail unwinding, so analysis saves
// it little, while the analyzer pays an identity-table lookup and closure
// chains on every application; the book's modest analyzer win assumed a
// naive tree-walking interpreter.

/** One engine's benchmark reading: the agreed answer and the median batch
 * time in nanoseconds. */
public data class Timing(
    public val value: Value,
    public val medianNanos: Double,
)

private const val DEFINITION = "(define (fib n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))"
private const val CALL = "(fib 12)"
private const val WARMUP_CALLS = 100
private const val TIMED_CALLS = 100
private const val SAMPLES = 7

/** The direct evaluator of 4.1.1: every evaluation re-dispatches on the
 * expression type. => a Timing with fib 12 and its median batch time */
public fun directEvaluatorTiming(): Timing {
    val env = setupEnvironment(OutputSink())
    return either {
        val define = parseExpr(readDatum(DEFINITION))
        val call = parseExpr(readDatum(CALL))
        val evaluator = Evaluator(env)
        evaluator.eval(define, env)
        Timing(evaluator.eval(call, env), medianNanos { evaluator.eval(call, env) })
    }.fold(
        { e -> throw IllegalStateException("benchmark run failed: ${formatError(e)}") },
        { it },
    )
}

/** The analyzer of 4.1.7: the syntactic work runs once, at analysis time;
 * execution calls the stored execution procedures. => a Timing with fib 12
 * and its median batch time */
public fun analyzerTiming(): Timing {
    val env = setupEnvironment(OutputSink())
    return either {
        val define = parseExpr(readDatum(DEFINITION))
        val call = parseExpr(readDatum(CALL))
        val analyzer = Analyzer(env)
        analyzer.eval(define, env)
        Timing(analyzer.eval(call, env), medianNanos { analyzer.eval(call, env) })
    }.fold(
        { e -> throw IllegalStateException("benchmark run failed: ${formatError(e)}") },
        { it },
    )
}

/** [WARMUP_CALLS] unmeasured calls, then the median of [SAMPLES] batches
 * of [TIMED_CALLS] measured calls. */
private inline fun medianNanos(batch: () -> Unit): Double {
    repeat(WARMUP_CALLS) { batch() }
    val samples = LongArray(SAMPLES)
    for (i in 0 until SAMPLES) {
        samples[i] = measureNanoTime { repeat(TIMED_CALLS) { batch() } }
    }
    samples.sort()
    return samples[SAMPLES / 2].toDouble()
}
