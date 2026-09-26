// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.30

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_30Test :
    FunSpec({
        test("the caught failures report through signal-error and a clean factorial still answers 120") {
            errorSignalingRuns() shouldBe
                listOf(
                    "operation failed: type error: car of 5",
                    ";;; EC-Eval input:",
                    "operation failed: division by zero",
                    ";;; EC-Eval input:",
                    "operation failed: unbound variable: no-such-variable",
                    ";;; EC-Eval input:",
                    "operation failed: arity mismatch: expected 2, given 1",
                    ";;; EC-Eval input:",
                    "end of input",
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "ok",
                    ";;; EC-Eval input:",
                    ";;; EC-Eval value:",
                    "120",
                    ";;; EC-Eval input:",
                )
        }
    })
