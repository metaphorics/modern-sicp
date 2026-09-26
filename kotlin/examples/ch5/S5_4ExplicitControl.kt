// SPDX-License-Identifier: GPL-3.0-only
// Section 5.4.4: running the evaluator. The book's session on the plain
// driver -- the definition, then the call, value 120 -- and the monitored
// driver's stack statistics the prose quotes: the definition costs three
// pushes, the call answers 144 pushes at depth 28.

package sicp.ch5.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch5.Evaluator
import sicp.ch5.baseEvaluatorController
import sicp.ch5.makeEvaluator
import sicp.ch5.monitoredEvaluatorController

private val factorialSource: String =
    """
    (define (factorial n)
      (if (= n 1)
          1
          (* (factorial (- n 1)) n)))
    (factorial 5)
    """.trimIndent()

private fun run(
    controller: List<sicp.runtime.Stmt>,
    source: String,
): List<String> =
    either {
        val evaluator = makeEvaluator(source, controller)
        evaluator.drive()
        evaluator.transcript
    }.fold(
        { e -> error("the replay failed: $e") },
        { it },
    )

private fun Evaluator.values(): List<String> = transcript.filterNot { it.startsWith(";;;") }

public class S5_4ExplicitControlTest :
    FunSpec({
        test("the plain driver runs the book's session") {
            run(baseEvaluatorController, "(+ 40 2)") shouldBe
                listOf(";;; EC-Eval input:", ";;; EC-Eval value:", "42", ";;; EC-Eval input:")
        }

        test("the definition answers ok and the call answers 120") {
            run(baseEvaluatorController, factorialSource) shouldBe
                listOf(
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "ok",
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "120",
                    ";;; EC-Eval input:",
                )
        }

        test("the monitored driver reports 3 pushes for the define and 144 at depth 28 for the call") {
            run(monitoredEvaluatorController, factorialSource) shouldBe
                listOf(
                    ";;; EC-Eval input:",
                    "(total-pushes = 3 maximum-depth = 3)",
                    ";;; EC-Eval value:",
                    "ok",
                    ";;; EC-Eval input:",
                    "(total-pushes = 144 maximum-depth = 28)",
                    ";;; EC-Eval value:",
                    "120",
                    ";;; EC-Eval input:",
                )
        }

        test("the 5.4.4 session: the define answers ok and the append answers the book's list") {
            run(
                baseEvaluatorController,
                """
                (define (append x y)
                  (if (null? x)
                      y
                      (cons (car x) (append (cdr x) y))))
                (append '(a b c) '(d e f))
                """.trimIndent(),
            ) shouldBe
                listOf(
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "ok",
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "(a b c d e f)",
                    ";;; EC-Eval input:",
                )
        }

        test("the values the drivers announce are the book's") {
            for (controller in listOf(baseEvaluatorController, monitoredEvaluatorController)) {
                val lines = run(controller, factorialSource)
                lines.filterNot { it.startsWith(";;;") || it.startsWith("(total-pushes") } shouldBe
                    listOf("ok", "120")
            }
        }
    })
