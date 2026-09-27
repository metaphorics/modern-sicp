// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.1.1, the core of the evaluator: eval as the clause
// chain over parsed expressions, apply over primitives and compound
// procedures, and the operand order the host fixes. The book's unknown-
// expression fault cannot fire in this edition -- the parser maps every
// well-formed datum onto a typed Expr node -- so malformed forms fail at
// parse time with the typed parse error instead.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.OutputSink
import sicp.ch4.evalTextEither
import sicp.ch4.runProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.SchemeError
import sicp.runtime.VInt
import sicp.runtime.VSym
import sicp.runtime.Value

/** The assert-on-success boundary: one expression text, its value. */
private fun eval(
    text: String,
    env: sicp.runtime.Env,
): Value = evalTextEither(text, env).fold({ e -> throw AssertionError(e.toString()) }, { it })

public class S4_1_1EvalApplyTest :
    FunSpec({
        val env = setupEnvironment(OutputSink())

        test("primitives and compound procedures compose") {
            eval("(+ 2 (* 3 4))", env) shouldBe VInt(14)
        }

        test("definitions and assignments answer the ok symbol") {
            eval("(define x 10)", env) shouldBe VSym("ok")
            eval("(set! x (+ x 5))", env) shouldBe VSym("ok")
            eval("x", env) shouldBe VInt(15)
        }

        test("cond covers test clauses and the else clause") {
            eval("(cond ((= 1 2) 'no) ((= 1 1) 'yes) (else 'maybe))", env) shouldBe VSym("yes")
            eval("(cond ((= 1 2) 'no) (else 'yes))", env) shouldBe VSym("yes")
        }

        test("let is a derived expression over lambda") {
            eval("(let ((x 3) (y 4)) (+ x y))", env) shouldBe VInt(7)
        }

        test("operands evaluate left to right, the host's fixed order") {
            val sink = OutputSink()
            val transcript =
                runProgram(
                    """
                    (define order '())
                    (define (note x) (set! order (cons x order)) x)
                    (cons (note 1) (note 2))
                    order
                    """.trimIndent(),
                    setupEnvironment(sink),
                    sink,
                )
            // the cons prints (1 . 2); note ran left to right, so order reads (2 1)
            transcript shouldBe "(1 . 2)\n(2 1)\n"
        }

        test("if skips the untaken branch") {
            val sink = OutputSink()
            val transcript =
                runProgram(
                    "(if (> 3 2) 1 (error \"not taken\"))",
                    setupEnvironment(sink),
                    sink,
                )
            transcript shouldBe "1\n"
        }

        test("an unknown expression fails typed, not silent") {
            evalTextEither("(1 2)", env).fold(
                { e -> (e is SchemeError.NotApplicable) shouldBe true },
                { },
            )
        }
    })
