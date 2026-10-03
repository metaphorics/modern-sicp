// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.38

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_38Test :
    FunSpec({
        test("Exercise 5.38: the counts and the n-ary runs pin the open coding").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            openCodedRuns().last() shouldBe "compiled and direct runs agree: true"
        }
    })
