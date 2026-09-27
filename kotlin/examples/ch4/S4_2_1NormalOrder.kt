// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.2.1, normal order and applicative order: the
// section's `try` and `unless` procedures under delayed arguments, with
// the strict base evaluator as the applicative-order contrast.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.LazyEvaluator
import sicp.ch4.OutputSink
import sicp.ch4.lazyTranscriptOn
import sicp.ch4.runProgram
import sicp.ch4.setupEnvironment

public class S4_2_1NormalOrderTest :
    FunSpec({
        test("the try session: the armed argument is never evaluated") {
            lazyTranscriptOn(
                ::LazyEvaluator,
                """
                (define (try a b)
                  (if (= a 0) 1 b))
                (try 0 (/ 1 0))
                """.trimIndent(),
            ) shouldBe "1\n"
        }

        test("the same call in an applicative-order language raises") {
            val sink = OutputSink()
            runProgram(
                """
                (define (try a b)
                  (if (= a 0) 1 b))
                (try 0 (/ 1 0))
                """.trimIndent(),
                setupEnvironment(sink),
                sink,
            ) shouldBe "Error: division by zero\n"
        }

        test("unless does useful work past an argument that would fault") {
            lazyTranscriptOn(
                ::LazyEvaluator,
                """
                (define a 12)
                (define b 0)
                (define (unless condition usual-value exceptional-value)
                  (if condition exceptional-value usual-value))
                (unless (= b 0)
                        (/ a b)
                        (begin (display "exception: returning 0")
                               0))
                """.trimIndent(),
            ) shouldBe "exception: returning 00\n"
        }

        test("the same unless under applicative order evaluates both arms") {
            val sink = OutputSink()
            runProgram(
                """
                (define a 12)
                (define b 0)
                (define (unless condition usual-value exceptional-value)
                  (if condition exceptional-value usual-value))
                (unless (= b 0)
                        (/ a b)
                        (begin (display "exception: returning 0")
                               0))
                """.trimIndent(),
                setupEnvironment(sink),
                sink,
            ) shouldBe "Error: division by zero\n"
        }
    })
