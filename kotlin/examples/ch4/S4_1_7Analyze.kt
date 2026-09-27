// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.7, separating syntactic analysis from execution:
// `analyze` compiles each expression once into an execution procedure and
// the analyzed run answers exactly as the direct evaluator does.

package sicp.ch4.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.Analyzer
import sicp.ch4.Evaluator
import sicp.ch4.OutputSink
import sicp.ch4.parseProgram
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.DefineE
import sicp.runtime.Env
import sicp.runtime.VInt

private const val FACTORIAL = "(define (factorial n) (if (= n 1) 1 (* n (factorial (- n 1)))))"

/** The whole text analyzed once, then run against an environment. */
private fun analyzeAndRun(
    text: String,
    env: Env,
): String {
    val analyzer = Analyzer(env)
    val sink = OutputSink()
    val out = StringBuilder()
    either {
        for (expr in parseProgram(readProgram(text))) {
            if (expr is DefineE) {
                analyzer.eval(expr, env) // a define prints nothing
                continue
            }
            out.append(sicp.ch4.printValue(analyzer.eval(expr, env))).append('\n')
        }
    }.fold(
        { e -> out.append("Error: ").append(sicp.ch4.formatError(e)).append('\n') },
        { },
    )
    return out.toString()
}

public class S4_1_7AnalyzeTest :
    FunSpec({
        test("the analyzed evaluator answers as the direct one") {
            val program = "$FACTORIAL\n(factorial 5)\n(factorial 10)"
            val direct = sicp.ch4.runProgram(program, setupEnvironment(OutputSink()), OutputSink())
            val analyzed = analyzeAndRun(program, setupEnvironment(OutputSink()))
            direct shouldBe "120\n3628800\n"
            analyzed shouldBe "120\n3628800\n"
        }

        test("the same execution procedure runs many times") {
            val env = setupEnvironment(OutputSink())
            val analyzer = Analyzer(env)
            either {
                for (expr in parseProgram(readProgram(FACTORIAL))) analyzer.eval(expr, env)
                val call = parseProgram(readProgram("(factorial 6)")).single()
                analyzer.eval(call, env) shouldBe VInt(720)
                analyzer.eval(call, env) shouldBe VInt(720) // no re-analysis needed
            }
        }

        test("the direct evaluator and the analyzer agree on data shapes") {
            val env = setupEnvironment(OutputSink())
            val program = "(cons (square 3) '())\n(list (quote a) (quote b))"
            val direct = sicp.ch4.runProgram("(define (square x) (* x x))\n$program", setupEnvironment(OutputSink()), OutputSink())
            val analyzed = analyzeAndRun("(define (square x) (* x x))\n$program", env)
            direct shouldBe "(9)\n(a b)\n"
            analyzed shouldBe direct
        }
    })
