// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 77

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe

public class E2_77Test :
    FunSpec({
        test("magnitude on the Figure 2.24 number dispatches exactly twice") {
            ex_2_77() shouldBe 2
        }

        test("the forwarded selectors answer at the representation level") {
            val table = NumTable()
            val reps = RepTable()
            installGenericArithmetic(table)
            installRepresentationPackages(reps)
            installComplexSelectors(table, reps, DispatchCounter())
            val z = Complex(Rect(3.0, 4.0))
            val result = arrow.core.raise.either { applyGeneric(table, "magnitude", listOf(z)) }
            result.getOrNull() shouldBe Real(5.0)
        }

        test("without Alyssa's install the complex level misses the selectors") {
            val table = NumTable()
            installGenericArithmetic(table)
            val z = Complex(Rect(3.0, 4.0))
            val result = arrow.core.raise.either { applyGeneric(table, "magnitude", listOf(z)) }
            result.leftOrNull() shouldBe GenError.NoMethod("magnitude", listOf("complex"))
        }
    })
