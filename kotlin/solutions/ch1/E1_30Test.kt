// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.30

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.long
import io.kotest.property.checkAll

private fun sumRecursive(
    term: (Long) -> Long,
    a: Long,
    next: (Long) -> Long,
    b: Long,
): Long = if (a > b) 0L else term(a) + sumRecursive(term, next(a), next, b)

public class E1_30Test :
    FunSpec({
        test("sum-cubes(1, 10) run iteratively still totals 3025") {
            ex_1_30() shouldBe 3025L
        }
        test("sumIterative agrees with the recursive sum of section 1.3.1 for every generated range") {
            checkAll(Arb.long(-20L, 20L), Arb.long(-20L, 20L)) { a, b ->
                sumIterative({ x -> x }, a, { x -> x + 1L }, b) shouldBe sumRecursive({ x -> x }, a, { x -> x + 1L }, b)
            }
        }
    })
