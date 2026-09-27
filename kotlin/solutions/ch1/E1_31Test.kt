// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.31

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.long
import io.kotest.property.checkAll

public class E1_31Test :
    FunSpec({
        test("factorial via product, and the Wallis approximation to pi with 1000 terms") {
            ex_1_31() shouldBe Pair(720L, 3.143160705532257)
        }
        test("product and productIterative agree for every generated range") {
            checkAll(Arb.long(1L, 8L), Arb.long(1L, 8L)) { a, b ->
                product({ x -> x }, a, { x -> x + 1L }, b) shouldBe productIterative({ x -> x }, a, { x -> x + 1L }, b)
            }
        }
    })
