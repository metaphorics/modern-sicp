// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.2

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
import sicp.runtime.VInt
import sicp.runtime.VNil
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

public class E42Test :
    FunSpec({
        test("Exercise 4.2a: a definition evaluates as an application of an unbound operator").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = ApplicationsFirst(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define x 3)") shouldBe Either.Left(SchemeError.Unbound("define"))
        }

        test("Exercise 4.2a: an assignment fails the same way, before its target is looked up").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = ApplicationsFirst(setupEnvironment(OutputSink()))
            runIn(evaluator, "(set! x 3)") shouldBe Either.Left(SchemeError.Unbound("set!"))
        }

        test("Exercise 4.2a: a procedure definition is caught by the application clause too").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = ApplicationsFirst(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define (f x) x) (f 2)") shouldBe Either.Left(SchemeError.Unbound("define"))
        }

        test("Exercise 4.2b: call-prefixed applications run").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = CallSyntax(setupEnvironment(OutputSink()))
            runIn(evaluator, "(call (lambda (x) (* x x)) 7)") shouldBe Either.Right(VInt(49))
        }

        test("Exercise 4.2b: bare combinations still apply and special forms still dispatch").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = CallSyntax(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define (f x) x) (f 3)") shouldBe Either.Right(VInt(3))
        }

        test("Exercise 4.2b: call sugar strips at every depth").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = CallSyntax(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define (twice f x) (call f (call f x))) (twice (lambda (y) (* y 10)) 3)") shouldBe
                Either.Right(VInt(300))
        }
    })
