// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.55

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_55Test :
    FunSpec({
        test("nested quotation remains explicit host data") {
            val outer = doubledQuoteAbracadabra as QuotationForm.Quoted
            val inner = outer.operand as QuotationForm.Quoted
            inner.operand shouldBe QuotationForm.Name("abracadabra")
        }
        test("inspecting one layer yields a quotation node, not the name") {
            ex_2_55() shouldBe QuotationForm.Quoted(QuotationForm.Name("abracadabra"))
        }
    })
