// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.32

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.long
import io.kotest.property.checkAll

public class E1_32Test :
    FunSpec({
        test("sum-cubes(1, 10) and factorial(6), both via accumulate") {
            ex_1_32() shouldBe Pair(3025L, 720L)
        }
        test("accumulate and accumulateIterative agree for every generated range") {
            checkAll(Arb.long(1L, 8L), Arb.long(1L, 8L)) { a, b ->
                accumulate({ x, y -> x + y }, 0L, { x -> x }, a, { x -> x + 1L }, b) shouldBe
                    accumulateIterative({ x, y -> x + y }, 0L, { x -> x }, a, { x -> x + 1L }, b)
            }
        }
    })
