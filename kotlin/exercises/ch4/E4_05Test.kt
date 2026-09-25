// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.5

package sicp.ch4.exercises

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe
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
    evaluator: WithArrow,
    program: String,
): Either<SchemeError, Value> =
    either {
        var last: Value = VNil
        for (expr in parseProgram(readProgram(program))) last = evaluator.eval(expr, evaluator.global)
        last
    }

public class E45Test :
    FunSpec({
        test("Exercise 4.5: the book's assoc example answers through the arrow").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithArrow(setupEnvironment(OutputSink()))
            runIn(
                evaluator,
                """
                (define (assoc key records)
                  (cond ((null? records) #f)
                        ((equal? key (car (car records))) (car records))
                        (else (assoc key (cdr records)))))
                (cond ((assoc 'b '((a 1) (b 2))) => cadr) (else 'no))
                """.trimIndent(),
            ) shouldBe Either.Right(VInt(2))
        }

        test("Exercise 4.5: the arrow test evaluates exactly once").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithArrow(setupEnvironment(OutputSink()))
            runIn(
                evaluator,
                """
                (define count 0)
                (define (assoc key records)
                  (cond ((null? records) #f)
                        ((equal? key (car (car records))) (car records))
                        (else (assoc key (cdr records)))))
                """.trimIndent(),
            ) shouldBe Either.Right(VSym("ok"))
            runIn(
                evaluator,
                "(cond ((begin (set! count (+ count 1)) (assoc 'b '((a 1) (b 2)))) => cadr) (else 'no))",
            ) shouldBe Either.Right(VInt(2))
            runIn(evaluator, "count") shouldBe Either.Right(VInt(1))
        }

        test("Exercise 4.5: the recipient receives the test's value, not its truth").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithArrow(setupEnvironment(OutputSink()))
            runIn(evaluator, "(cond (#f 1) (99 => (lambda (v) (* v 2))))") shouldBe Either.Right(VInt(198))
        }

        test("Exercise 4.5: plain clauses still chain in order").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithArrow(setupEnvironment(OutputSink()))
            runIn(evaluator, "(cond ((= 1 2) 'first) ((= 1 1) 'second) (else 'third))") shouldBe
                Either.Right(VSym("second"))
        }

        test("Exercise 4.5: no match with no else answers false").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val evaluator = WithArrow(setupEnvironment(OutputSink()))
            runIn(evaluator, "(cond (#f 'never) ((= 1 2) => (lambda (v) v)))") shouldBe Either.Right(VBool(false))
        }
    })
