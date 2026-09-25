// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.3.1, amb and search: the driver-loop interaction
// the section opens with, the six values of a pair of choices, and the
// unbounded `an-integer-starting-from` generator.

package sicp.ch4.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.AmbEvaluator
import sicp.ch4.ambDriver
import sicp.ch4.printValue

private val PRELUDE =
    """
    (define (require p) (if (not p) (amb)))
    (define (an-element-of items)
      (require (not (null? items)))
      (amb (car items) (an-element-of (cdr items))))
    (define (an-integer-between low high)
      (require (<= low high))
      (amb low (an-integer-between (+ low 1) high)))
    (define (an-integer-starting-from n)
      (amb n (an-integer-starting-from (+ n 1))))
    (define (divides? a b) (= (remainder b a) 0))
    (define (find-divisor n test)
      (cond ((> (* test test) n) n)
            ((divides? test n) test)
            (else (find-divisor n (+ test 1)))))
    (define (smallest-divisor n) (find-divisor n 2))
    (define (prime? n) (= n (smallest-divisor n)))
    (define (prime-sum-pair list1 list2)
      (let ((a (an-element-of list1))
            (b (an-element-of list2)))
        (require (prime? (+ a b)))
        (list a b)))
    """.trimIndent()

private fun newDriver() = ambDriver(::AmbEvaluator, PRELUDE)

/** Collects the first [count] answers of [query] as printed strings. */
private fun firstAnswers(
    query: String,
    count: Int,
): List<String> =
    either {
        val driver = newDriver()
        val answers = mutableListOf<String>()
        var next = driver.solve(query)
        while (next != null && answers.size < count) {
            answers.add(printValue(next))
            next = driver.tryAgain()
        }
        answers
    }.fold(
        { e -> throw AssertionError(e.toString()) },
        { it },
    )

public class S4_3_1AmbAndSearchTest :
    FunSpec({
        test("the driver loop answers (3 20) first, then (3 110), then (8 35)") {
            val driver = newDriver()
            driver.input("(prime-sum-pair '(1 3 5 8) '(20 35 110))") shouldBe
                """
                ;;; Amb-Eval input:
                (prime-sum-pair '(1 3 5 8) '(20 35 110))
                ;;; Starting a new problem
                ;;; Amb-Eval value:
                (3 20)
                """.trimIndent() + "\n"
            driver.input("try-again") shouldBe
                """
                ;;; Amb-Eval input:
                try-again
                ;;; Amb-Eval value:
                (3 110)
                """.trimIndent() + "\n"
            driver.input("try-again") shouldBe
                """
                ;;; Amb-Eval input:
                try-again
                ;;; Amb-Eval value:
                (8 35)
                """.trimIndent() + "\n"
        }

        test("exhaustion reports the problem, and a new one starts over") {
            val driver = newDriver()
            driver.input("(prime-sum-pair '(1 3 5 8) '(20 35 110))")
            repeat(3) { driver.input("try-again") }
            driver.input("try-again") shouldBe
                """
                ;;; Amb-Eval input:
                try-again
                ;;; There is no current problem
                """.trimIndent() + "\n"
            driver.input("(prime-sum-pair '(19 27 30) '(11 36 58))") shouldBe
                """
                ;;; Amb-Eval input:
                (prime-sum-pair '(19 27 30) '(11 36 58))
                ;;; Starting a new problem
                ;;; Amb-Eval value:
                (30 11)
                """.trimIndent() + "\n"
        }

        test("a fresh try-again finds no current problem") {
            ambDriver(::AmbEvaluator, "(define (ignored) 1)\n").tryAgainRound() shouldBe
                ";;; There is no current problem\n"
        }

        test("a pair of choices has six possible values") {
            firstAnswers("(list (amb 1 2 3) (amb 'a 'b))", 6) shouldBe
                listOf("(1 a)", "(1 b)", "(2 a)", "(2 b)", "(3 a)", "(3 b)")
        }

        test("an unbounded generator keeps delivering the next integer") {
            firstAnswers("(an-integer-starting-from 5)", 3) shouldBe listOf("5", "6", "7")
        }
    })
