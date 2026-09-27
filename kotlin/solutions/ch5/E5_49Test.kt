// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.49

package sicp.ch5.solutions

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E5_49Test :
    FunSpec({
        test("the loop compiles every form and prints each value") {
            readCompileExecutePrint() shouldBe
                listOf(
                    ";;; EC-Eval input: ;;; EC-Eval value: ok",
                    ";;; EC-Eval input: ;;; EC-Eval value: 144",
                    ";;; EC-Eval input: ;;; EC-Eval value: ok",
                    ";;; EC-Eval input: ;;; EC-Eval value: 882",
                )
        }
    })
