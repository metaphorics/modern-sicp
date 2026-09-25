// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.22

package sicp.ch4.solutions

import arrow.core.raise.either
import sicp.ch4.Analyzer
import sicp.ch4.Exec
import sicp.ch4.OutputSink
import sicp.ch4.formatError
import sicp.ch4.parseProgram
import sicp.ch4.printValue
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.Expr
import sicp.runtime.LetE

// Exercise 4.22: `let` in the analyzed evaluator. The analyzer seam is
// [Analyzer.analyze]: [LetAnalyzer] checks for a [LetE] node before falling
// back to the base chain, and because every analysis of a subexpression
// recurses through `analyze`, the `let` clause fires at every nesting depth.
//
// The handling is a direct analyzed `let` rather than the 4.6 derivation to
// a lambda application: each initializer is analyzed once here, the body is
// analyzed once by [analyzeSequence], and the returned execution procedure
// only extends the frame with the initializer values and runs the body --
// the same shape as `analyze-lambda`, with nothing re-analyzed at
// execution time.

/**
 * The analyzer with `let` handled in the analysis phase.
 */
public class LetAnalyzer(
    global: Env,
) : Analyzer(global) {
    override fun analyze(expr: Expr): Exec =
        when (expr) {
            is LetE -> analyzedLet(expr)
            else -> super.analyze(expr)
        }

    /** The direct analyzed `let`: initializers and body analyzed once, at
     * analysis time; execution only binds and runs. */
    private fun analyzedLet(expr: LetE): Exec {
        val initProcs = expr.bindings.map { analyze(it.value) }
        val bodyProc = analyzeSequence(expr.body)
        val names = expr.bindings.map { it.name }
        return { env ->
            val frame = Env.extend(names, initProcs.map { it(env) }, env)
            bodyProc(frame)
        }
    }
}

/** The shadowing probe: the plain lookup, an inner `let` shadowing the
 * outer binding, an initializer reading the outer binding, and a `let`
 * inside an analyzed procedure body. */
private val PROGRAM: String =
    """
    (let ((x 7)) x)
    (let ((x 5)) (let ((x 2)) x))
    (let ((x 5)) (let ((y x)) y))
    (define (f x) (let ((y (* x x))) (+ y 1)))
    (f 10)
    """.trimIndent()

/** Runs [PROGRAM] on [LetAnalyzer] and returns the printer-contract
 * transcript. => 7, 2, 5, then 101 */
public fun letTranscript(): String = runOn(::LetAnalyzer, PROGRAM)

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
