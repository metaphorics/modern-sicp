// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.51

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_51Test :
    FunSpec({
        test("Exercise 4.51: permanent-set! accumulates across answers and survives") {
            permanentSetTranscript() shouldBe
                """
                ;;; Amb-Eval input:
                (let ((x (an-element-of '(a b c)))
                      (y (an-element-of '(a b c))))
                  (permanent-set! count (+ count 1))
                  (require (not (eq? x y)))
                  (list x y count))
                ;;; Starting a new problem
                ;;; Amb-Eval value:
                (a b 2)
                ;;; Amb-Eval input:
                try-again
                ;;; Amb-Eval value:
                (a c 3)
                ;;; Amb-Eval input:
                try-again
                ;;; Amb-Eval value:
                (b a 4)
                ;;; Amb-Eval input:
                count
                ;;; Starting a new problem
                ;;; Amb-Eval value:
                4
                """.trimIndent() + "\n"
        }

        test("Exercise 4.51: set! rolls each failed trial back") {
            setBangTranscript() shouldBe
                """
                ;;; Amb-Eval input:
                (let ((x (an-element-of '(a b c)))
                      (y (an-element-of '(a b c))))
                  (set! count (+ count 1))
                  (require (not (eq? x y)))
                  (list x y count))
                ;;; Starting a new problem
                ;;; Amb-Eval value:
                (a b 1)
                ;;; Amb-Eval input:
                try-again
                ;;; Amb-Eval value:
                (a c 1)
                ;;; Amb-Eval input:
                try-again
                ;;; Amb-Eval value:
                (b a 1)
                ;;; Amb-Eval input:
                count
                ;;; Starting a new problem
                ;;; Amb-Eval value:
                0
                """.trimIndent() + "\n"
        }
    })
