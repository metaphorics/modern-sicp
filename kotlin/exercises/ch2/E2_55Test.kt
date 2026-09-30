// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.55

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.core.test.Enabled
import io.kotest.matchers.shouldBe

public class E2_55Test :
    FunSpec({
        test("Exercise 2.55 returns the inner quotation node").config(
            enabledOrReasonIf = { Enabled.disabled("pending solution") },
        ) {
            ex_2_55() shouldBe QuotationForm.Quoted(QuotationForm.Name("abracadabra"))
        }
    })
