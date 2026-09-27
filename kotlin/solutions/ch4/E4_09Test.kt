// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.9

package sicp.ch4.solutions

import arrow.core.Either
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
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
    evaluator: WithLoops,
    program: String,
): Either<SchemeError, Value> =
    either {
        var last: Value = VNil
        for (expr in parseProgram(readProgram(program))) last = evaluator.eval(expr, evaluator.global)
        last
    }

public class E49Test :
    FunSpec({
        test("Exercise 4.9: while accumulates the sum of 1 to 5") {
            val evaluator = WithLoops(setupEnvironment(OutputSink()))
            runIn(
                evaluator,
                """
                (define i 1)
                (define sum 0)
                (while (<= i 5) (set! sum (+ sum i)) (set! i (+ i 1)))
                sum
                """.trimIndent(),
            ) shouldBe Either.Right(VInt(15))
        }

        test("Exercise 4.9: a while body never runs when the test starts false") {
            val evaluator = WithLoops(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define k 0) (while #f (set! k 1)) k") shouldBe Either.Right(VInt(0))
        }

        test("Exercise 4.9: until accumulates until the test holds") {
            val evaluator = WithLoops(setupEnvironment(OutputSink()))
            runIn(
                evaluator,
                """
                (define j 8)
                (define product 1)
                (until (> j 12) (set! product (* product j)) (set! j (+ j 1)))
                product
                """.trimIndent(),
            ) shouldBe Either.Right(VInt(95040))
            runIn(evaluator, "j") shouldBe Either.Right(VInt(13))
        }

        test("Exercise 4.9: an until body never runs when the test starts true") {
            val evaluator = WithLoops(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define m 0) (until #t (set! m 1)) m") shouldBe Either.Right(VInt(0))
        }

        test("Exercise 4.9: the loop self-call is a tail call") {
            val evaluator = WithLoops(setupEnvironment(OutputSink()))
            runIn(evaluator, "(define c 0) (while (< c 100000) (set! c (+ c 1))) c") shouldBe
                Either.Right(VInt(100000))
        }

        test("Exercise 4.9: loops nest, each wrapper frame holding its own loop") {
            val evaluator = WithLoops(setupEnvironment(OutputSink()))
            runIn(
                evaluator,
                """
                (define total 0)
                (define outer 1)
                (while (<= outer 3)
                  (define inner 1)
                  (while (<= inner outer)
                    (set! total (+ total 1))
                    (set! inner (+ inner 1)))
                  (set! outer (+ outer 1)))
                total
                """.trimIndent(),
            ) shouldBe Either.Right(VInt(6))
        }
    })
