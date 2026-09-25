// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.5, data as programs: the evaluator as a universal
// machine. The factorial program is ordinary list data -- the example
// takes it apart with car and cdr, then feeds the very same structure to
// the evaluator and watches it compute.

package sicp.ch4.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.OutputSink
import sicp.ch4.evalText
import sicp.ch4.runProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.VInt
import sicp.runtime.VPair
import sicp.runtime.VSym

private const val FACTORIAL_PROGRAM = "(define (factorial n) (if (= n 1) 1 (* n (factorial (- n 1)))))"

public class S4_1_5DataAsProgramsTest :
    FunSpec({
        test("the factorial program is a list: its pieces are car and cdr") {
            val env = setupEnvironment(OutputSink())
            either {
                val datum = evalText("'$FACTORIAL_PROGRAM", env) as VPair
                datum.car shouldBe VSym("define")
                val signature = (datum.cdr as VPair).car as VPair
                signature.car shouldBe VSym("factorial")
                (signature.cdr as VPair).car shouldBe VSym("n")
            }
        }

        test("feed the program to the evaluator and it computes") {
            val sink = OutputSink()
            val transcript =
                runProgram(
                    """
                    $FACTORIAL_PROGRAM
                    (factorial 5)
                    (factorial 10)
                    """.trimIndent(),
                    setupEnvironment(sink),
                    sink,
                )
            transcript shouldBe "120\n3628800\n"
        }

        test("data and programs are the same stuff, both directions") {
            // the program read as data prints back as the same surface text
            val sink = OutputSink()
            val env = setupEnvironment(sink)
            val printed =
                either {
                    sicp.ch4.printValue(evalText("'$FACTORIAL_PROGRAM", env))
                }.fold({ throw AssertionError() }, { it })
            printed shouldBe FACTORIAL_PROGRAM
            // and evaluating the very same text as a program installs factorial
            runProgram(FACTORIAL_PROGRAM, env, sink) shouldBe ""
            val f = either { env.lookup("factorial") }.fold({ throw AssertionError() }, { it })
            (f is sicp.runtime.VProc) shouldBe true
        }
    })
