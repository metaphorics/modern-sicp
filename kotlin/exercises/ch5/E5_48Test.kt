// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.48

package sicp.ch5.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E5_48Test :
    FunSpec({
        test("Exercise 5.48: the two-machine session answers ok then 120").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            compileAndRunSession().last() shouldBe "the two runs agree: true"
        }
    })
