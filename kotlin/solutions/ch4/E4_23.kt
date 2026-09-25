// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.23

package sicp.ch4.solutions

import arrow.core.raise.either
import kotlinx.collections.immutable.PersistentList
import sicp.ch4.Analyzer
import sicp.ch4.Exec
import sicp.ch4.OutputSink
import sicp.ch4.fail
import sicp.ch4.formatError
import sicp.ch4.parseExpr
import sicp.ch4.parseProgram
import sicp.ch4.printValue
import sicp.ch4.readDatum
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.SchemeError

// Exercise 4.23: Alyssa's `analyze-sequence`. The library's
// [Analyzer.analyzeSequence] is the text's version: every body expression
// is analyzed at analysis time and the execution procedures are composed
// before any call. [AlyssaSequence] overrides the seam with her version:
// only the FIRST expression of a sequence is analyzed at analysis time,
// and the returned closure re-runs `execute-sequence` on the tail at
// EXECUTION time, re-analyzing every later expression on each execution.
// The results are identical; the work is not.

/**
 * The analyzer carrying Alyssa's looping `execute-sequence`.
 */
public open class AlyssaSequence(
    global: Env,
) : Analyzer(global) {
    override fun analyzeSequence(actions: PersistentList<Expr>): Exec {
        if (actions.isEmpty()) {
            return { fail(SchemeError.TypeMismatch("Empty sequence: ANALYZE")) }
        }
        return executeSequence(actions)
    }

    /** Alyssa's loop: the one-expression clause returns the body's own
     * execution procedure; the multi-expression clause analyzes only the
     * first expression and defers the tail to every execution. */
    private fun executeSequence(exps: PersistentList<Expr>): Exec =
        if (exps.size == 1) {
            analyze(exps.first())
        } else {
            val firstProc = analyze(exps.first())
            val rest = exps.removingAt(0)
            val tail: Exec = { env ->
                firstProc(env)
                executeSequence(rest)(env) // the tail re-analyzed per execution
            }
            tail
        }
}

/** The value probe: a two-expression body whose answer is the tail's, so a
 * sequence that dropped the tail would answer the wrong value. */
private val PROBE: String =
    """
    (define (f x) (set! x (* x 2)) x)
    (f 10)
    (f 30)
    """.trimIndent()

/** One count of how many [Analyzer.analyze] calls a run makes: how many
 * expressions were analyzed after the definition, and how many per call. */
public data class AnalysisProfile(
    /** Hook calls while defining the probe procedure. */
    public val atDefinition: Int,
    /** Hook calls while evaluating one call of it. */
    public val perCall: Int,
)

/** Both profiles for one analyzer: a two-expression body and a
 * one-expression body, the comparison 4.23 asks for. */
public data class SequenceProfiles(
    public val twoExpressions: AnalysisProfile,
    public val oneExpression: AnalysisProfile,
)

/** The text analyzer on the value probe. => 20, then 60 */
public fun textSequenceTranscript(): String = runOn(::Analyzer, PROBE)

/** Alyssa's analyzer on the value probe: the same transcript. => 20, then 60 */
public fun alyssaSequenceTranscript(): String = runOn(::AlyssaSequence, PROBE)

/** The text analyzer's analysis counts. */
public fun textSequenceProfile(): SequenceProfiles =
    SequenceProfiles(
        twoExpressions = profile(::CountingText, TWO_EXPRESSION_BODY, ONE_CALL),
        oneExpression = profile(::CountingText, ONE_EXPRESSION_BODY, ONE_CALL_ONE_EXPRESSION),
    )

/** Alyssa's analyzer's analysis counts. */
public fun alyssaSequenceProfile(): SequenceProfiles =
    SequenceProfiles(
        twoExpressions = profile(::CountingAlyssa, TWO_EXPRESSION_BODY, ONE_CALL),
        oneExpression = profile(::CountingAlyssa, ONE_EXPRESSION_BODY, ONE_CALL_ONE_EXPRESSION),
    )

private const val TWO_EXPRESSION_BODY = "(define (f x) x x)"
private const val ONE_EXPRESSION_BODY = "(define (g y) y)"
private const val ONE_CALL = "(f 10)"
private const val ONE_CALL_ONE_EXPRESSION = "(g 5)"

/** The analyzer driver: defines print nothing, every other top-level form
 * prints one value line. */
private fun runOn(
    analyzerFactory: (Env) -> Analyzer,
    text: String,
): String {
    val sink = OutputSink()
    val env = setupEnvironment(sink)
    val analyzer = analyzerFactory(env)
    either {
        for (expr in parseProgram(readProgram(text))) {
            if (expr is DefineE) {
                analyzer.eval(expr, env) // a define prints nothing
                continue
            }
            sink.line(printValue(analyzer.eval(expr, env)))
        }
    }.fold(
        { e -> sink.line("Error: ${formatError(e)}") },
        { },
    )
    return sink.toString()
}

/** Counts [Analyzer.analyze] hook calls around the definition and around
 * one call of the defined procedure. */
private fun profile(
    analyzerFactory: (Env, Tally) -> Analyzer,
    definition: String,
    call: String,
): AnalysisProfile {
    val env = setupEnvironment(OutputSink())
    val tally = Tally()
    val analyzer = analyzerFactory(env, tally)
    return either {
        analyzer.eval(parseExpr(readDatum(definition)), env)
        val atDefinition = tally.n
        analyzer.eval(parseExpr(readDatum(call)), env)
        AnalysisProfile(atDefinition, tally.n - atDefinition)
    }.fold(
        { e -> throw IllegalStateException("profile run failed: ${formatError(e)}") },
        { it },
    )
}

/** Shared counter the counting analyzers bump from the [Analyzer.analyze]
 * hook; every analyzed expression passes through it exactly once per
 * analysis, so the tally is the analysis work of the run. */
private class Tally {
    var n: Int = 0
}

private class CountingText(
    global: Env,
    private val tally: Tally,
) : Analyzer(global) {
    override fun analyze(expr: Expr): Exec {
        tally.n += 1
        return super.analyze(expr)
    }
}

private class CountingAlyssa(
    global: Env,
    private val tally: Tally,
) : AlyssaSequence(global) {
    override fun analyze(expr: Expr): Exec {
        tally.n += 1
        return super.analyze(expr)
    }
}
