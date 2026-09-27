// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, section 4.3.3, implementing the amb evaluator: the engine's
// observable contract -- the undo trail that rolls a `set!` back when the
// branch dies, `try-again` resuming the deepest pending choice, and an
// object fault aborting the search.

package sicp.ch4.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch4.AmbEvaluator
import sicp.ch4.ambDriver

private val PRELUDE =
    """
    (define (require p) (if (not p) (amb)))
    (define (an-element-of items)
      (require (not (null? items)))
      (amb (car items) (an-element-of (cdr items))))
    (define traced 0)
    """.trimIndent()

public class S4_3_3EngineTest :
    FunSpec({
        test("a set! inside a dying branch is rolled back by the undo trail") {
            val driver = ambDriver(::AmbEvaluator, PRELUDE)
            driver.input("(let ((x (amb 1 2)))\n  (set! traced x)\n  (require (= x 2))\n  traced)") shouldBe
                """
                ;;; Amb-Eval input:
                (let ((x (amb 1 2)))
                  (set! traced x)
                  (require (= x 2))
                  traced)
                ;;; Starting a new problem
                ;;; Amb-Eval value:
                2
                """.trimIndent() + "\n"
            driver.input("try-again") shouldBe
                """
                ;;; Amb-Eval input:
                try-again
                ;;; There are no more values of
                the pending problem
                """.trimIndent() + "\n"
            driver.input("traced") shouldBe
                """
                ;;; Amb-Eval input:
                traced
                ;;; Starting a new problem
                ;;; Amb-Eval value:
                0
                """.trimIndent() + "\n"
        }

        test("try-again resumes the deepest pending choice, not the outermost") {
            val driver = ambDriver(::AmbEvaluator, PRELUDE)
            driver.input("(list (amb 1 2) (amb 'a 'b))")
            driver.input("try-again") shouldBe
                """
                ;;; Amb-Eval input:
                try-again
                ;;; Amb-Eval value:
                (1 b)
                """.trimIndent() + "\n"
            driver.input("try-again") shouldBe
                """
                ;;; Amb-Eval input:
                try-again
                ;;; Amb-Eval value:
                (2 a)
                """.trimIndent() + "\n"
        }

        test("an object fault aborts the search as a typed Error line") {
            val driver = ambDriver(::AmbEvaluator, PRELUDE)
            driver.input("(car (amb 1 2))") shouldBe
                """
                ;;; Amb-Eval input:
                (car (amb 1 2))
                ;;; Starting a new problem
                Error: type mismatch: car of a non-pair: 1
                """.trimIndent() + "\n"
            driver.input("try-again") shouldBe
                """
                ;;; Amb-Eval input:
                try-again
                ;;; There is no current problem
                """.trimIndent() + "\n"
        }
    })
