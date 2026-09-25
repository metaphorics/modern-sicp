// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.3

package sicp.ch4.solutions

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.OutputSink
import sicp.ch4.formatError
import sicp.ch4.parseProgram
import sicp.ch4.printValue
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.AppE
import sicp.runtime.IfE
import sicp.runtime.Key
import sicp.runtime.SchemeError
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VSym
import sicp.runtime.Value

/** Runs the top-level forms of `program` on the table-driven evaluator,
 * answering the last value or the typed fault. */
private fun runIn(
    evaluator: TableDriven,
    program: String,
): Either<SchemeError, Value> =
    either {
        var last: Value = VNil
        for (expr in parseProgram(readProgram(program))) last = evaluator.eval(expr, evaluator.global)
        last
    }

/** The printer-contract answer line of one run: the value's printed form,
 * or the `Error:` line. */
private fun runPrinted(
    evaluator: TableDriven,
    program: String,
): String = runIn(evaluator, program).fold({ e -> "Error: ${formatError(e)}" }, { printValue(it) })

public class E43Test :
    FunSpec({
        test("Exercise 4.3: cond dispatches through the table") {
            val evaluator = TableDriven(setupEnvironment(OutputSink()))
            runPrinted(evaluator, "(cond ((= 1 2) 'no) (else 'yes))") shouldBe "yes"
        }

        test("Exercise 4.3: definitions and applications run through table clauses") {
            val evaluator = TableDriven(setupEnvironment(OutputSink()))
            runPrinted(evaluator, "(define (square x) (* x x)) (square 7)") shouldBe "49"
        }

        test("Exercise 4.3: quote answers its datum through the table") {
            val evaluator = TableDriven(setupEnvironment(OutputSink()))
            runPrinted(evaluator, "(quote (a b))") shouldBe "(a b)"
        }

        test("Exercise 4.3: variables and self-evaluating expressions stay residual tests") {
            val evaluator = TableDriven(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define seven 7) (+ seven 12)") shouldBe Either.Right(VInt(19))
        }

        test("Exercise 4.3: a clause installed after construction extends the language") {
            val evaluator = TableDriven(setupEnvironment(OutputSink()))
            evaluator.put(Key.Sym("eval"), evalTagKey(listOf("unless"))) { _, expr, env ->
                val operands = (expr as AppE).operands
                evaluator.eval(IfE(operands[0], operands[2], operands[1]), env)
            }
            runPrinted(evaluator, "(unless (= 1 2) 'ran 'no)") shouldBe "ran"
            runPrinted(evaluator, "(unless (= 1 1) 'ran 'no)") shouldBe "no"
        }

        test("Exercise 4.3: a later put overwrites an installed clause") {
            val evaluator = TableDriven(setupEnvironment(OutputSink()))
            evaluator.put(Key.Sym("eval"), evalTagKey(listOf("quote"))) { _, _, _ ->
                VSym("replaced")
            }
            runPrinted(evaluator, "(quote (a b))") shouldBe "replaced"
        }
    })
