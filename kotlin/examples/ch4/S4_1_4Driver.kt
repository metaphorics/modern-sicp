// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.4, running the evaluator as a program: the
// primitive table, the global environment, and the driver loop. The
// book's sample interaction -- append defined, then applied -- is the
// pin, with the driver's transcript recorded per the printer contract.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.OutputSink
import sicp.ch4.defaultPrimitives
import sicp.ch4.runProgram
import sicp.ch4.setupEnvironment

public class S4_1_4DriverTest :
    FunSpec({
        test("the book's sample interaction: append as data, then applied") {
            val sink = OutputSink()
            val transcript =
                runProgram(
                    """
                    (define (append x y)
                      (if (null? x)
                          y
                          (cons (car x) (append (cdr x) y))))
                    (append '(a b c) '(d e f))
                    """.trimIndent(),
                    setupEnvironment(sink),
                    sink,
                )
            // the define prints nothing; the call prints its value
            transcript shouldBe "(a b c d e f)\n"
        }

        test("the primitive table binds the grammar's core plus the corpus additions") {
            val names = defaultPrimitives(OutputSink()).map { it.name }
            for (required in listOf(
                "car",
                "cdr",
                "cons",
                "null?",
                "+",
                "-",
                "*",
                "/",
                "error",
                "display",
                "newline",
                "apply",
                "map",
                "length",
                "set-car!",
                "set-cdr!",
                "cadr",
                "cadddr",
                "exact->inexact",
            )) {
                (required in names) shouldBe true
            }
        }

        test("display and newline write their side effect, with no value line") {
            val sink = OutputSink()
            val transcript =
                runProgram(
                    """
                    (display (list 1 2 3))
                    (newline)
                    (display 'symbol)
                    (newline)
                    42
                    """.trimIndent(),
                    setupEnvironment(sink),
                    sink,
                )
            transcript shouldBe "(1 2 3)\nsymbol\n42\n"
        }

        test("an error line stops the program") {
            val sink = OutputSink()
            val transcript =
                runProgram(
                    """
                    (+ 1 2)
                    (error "halted here:" 'irritant)
                    (+ 3 4)
                    """.trimIndent(),
                    setupEnvironment(sink),
                    sink,
                )
            transcript shouldBe "3\nError: halted here: irritant\n"
        }
    })
