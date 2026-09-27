// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.23

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_23Test :
    FunSpec({
        test("cond dispatches through cond->if, a bodyless clause answers its test, let becomes a lambda application") {
            derivedExpressionRuns() shouldBe
                listOf(
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "ok",
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "zero",
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "one",
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "many",
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "#f",
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "6",
                    ";;; EC-Eval input:",
                )
        }
    })
