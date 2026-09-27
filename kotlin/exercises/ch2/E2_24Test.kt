// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.24

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_24Test :
    FunSpec({
        test("Exercise 2.24: the nested vlist prints (1 (2 (3 4)))").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_24() shouldBe "(1 (2 (3 4)))"
        }
    })
