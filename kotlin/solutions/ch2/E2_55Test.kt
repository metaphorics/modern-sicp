// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.55 (replaced)

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VSym

public class E2_55Test :
    FunSpec({
        test("the doubled-quote value prints as (quote (quote abracadabra))") {
            doubledQuoteAbracadabra.toString() shouldBe "(quote (quote abracadabra))"
        }
        test("its car is the symbol quote, not abracadabra") {
            ex_2_55() shouldBe VSym("quote")
        }
    })
