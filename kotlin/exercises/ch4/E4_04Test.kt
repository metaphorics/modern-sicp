// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.4

package sicp.ch4.exercises

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
import sicp.ch4.Evaluator
import sicp.ch4.OutputSink
import sicp.ch4.parseProgram
import sicp.ch4.readProgram
import sicp.ch4.setupEnvironment
import sicp.runtime.SchemeError
import sicp.runtime.VBool
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VSym
import sicp.runtime.Value

/** Runs the top-level forms of `program` on `evaluator`, answering the last
 * value or the typed fault. */
private fun runIn(
    evaluator: Evaluator,
    program: String,
): Either<SchemeError, Value> =
    either {
        var last: Value = VNil
        for (expr in parseProgram(readProgram(program))) last = evaluator.eval(expr, evaluator.global)
        last
    }

public class E44Test :
    FunSpec({
        test("Exercise 4.4: the book's and examples").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithAndOr(setupEnvironment(OutputSink()))
            runIn(evaluator, "(and (= 2 2) (> 2 1))") shouldBe Either.Right(VBool(true))
            runIn(evaluator, "(and)") shouldBe Either.Right(VBool(true))
            runIn(evaluator, "(and 1 2 3)") shouldBe Either.Right(VInt(3))
            runIn(evaluator, "(and 1 #f (error \"no operand may evaluate\"))") shouldBe Either.Right(VBool(false))
        }

        test("Exercise 4.4: the book's or examples").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithAndOr(setupEnvironment(OutputSink()))
            runIn(evaluator, "(or (= 2 2) (> 2 1))") shouldBe Either.Right(VBool(true))
            runIn(evaluator, "(or)") shouldBe Either.Right(VBool(false))
            runIn(evaluator, "(or #f 7 (error \"no operand may evaluate\"))") shouldBe Either.Right(VInt(7))
            runIn(evaluator, "(or 'a 'b)") shouldBe Either.Right(VSym("a"))
        }

        test("Exercise 4.4: and answers the last value, or the first true one").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithAndOr(setupEnvironment(OutputSink()))
            runIn(evaluator, "(and #f 'x)") shouldBe Either.Right(VBool(false))
            runIn(evaluator, "(or 'b 'c)") shouldBe Either.Right(VSym("b"))
        }

        test("Exercise 4.4: the forms re-enter the whole evaluator, nested").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithAndOr(setupEnvironment(OutputSink()))
            runIn(evaluator, "(and (or #f 2) (if #t 40 0) (+ 1 1))") shouldBe Either.Right(VInt(2))
        }
    })
