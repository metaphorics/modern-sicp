// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.27

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E3_27Test :
    FunSpec({
        test("Exercise 3.27: the memoized Fibonacci answers in linear computes").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            val memoFib = makeMemoFib()
            memoFib(10L) shouldBe 55L
            memoFib(30L) shouldBe 832_040L
        }
    })
