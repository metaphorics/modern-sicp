// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.52

package sicp.ch4.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E4_52Test :
    FunSpec({
        test("Exercise 4.52: all odd means the alternative answers, then exhaustion") {
            ifFailAllOddTranscript() shouldBe
                """
                ;;; Amb-Eval input:
                (if-fail (let ((x (an-element-of '(1 3 5))))
                           (require (even? x))
                           x)
                         'all-odd)
                ;;; Starting a new problem
                ;;; Amb-Eval value:
                all-odd
                ;;; Amb-Eval input:
                try-again
                ;;; There are no more values of
                the pending problem
                """.trimIndent() + "\n"
        }

        test("Exercise 4.52: with 8 available, the value answers first and all-odd second") {
            ifFailEightTranscript() shouldBe
                """
                ;;; Amb-Eval input:
                (if-fail (let ((x (an-element-of '(1 3 5 8))))
                           (require (even? x))
                           x)
                         'all-odd)
                ;;; Starting a new problem
                ;;; Amb-Eval value:
                8
                ;;; Amb-Eval input:
                try-again
                ;;; Amb-Eval value:
                all-odd
                ;;; Amb-Eval input:
                try-again
                ;;; There are no more values of
                the pending problem
                """.trimIndent() + "\n"
        }
    })
