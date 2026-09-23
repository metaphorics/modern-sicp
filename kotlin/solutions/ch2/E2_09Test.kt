// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.9

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.double
import io.kotest.property.arbitrary.filter
import io.kotest.property.checkAll

public class E2_09Test :
    FunSpec({
        test("the width of a sum is exactly the sum of the widths, for many generated intervals") {
            checkAll(
                Arb.double(-100.0..100.0).filter { it.isFinite() },
                Arb.double(0.0..50.0).filter { it.isFinite() },
                Arb.double(-100.0..100.0).filter { it.isFinite() },
                Arb.double(0.0..50.0).filter { it.isFinite() },
            ) { cx, wx, cy, wy ->
                val x = makeInterval(cx - wx, cx + wx)
                val y = makeInterval(cy - wy, cy + wy)
                width(addInterval(x, y)) shouldBe (width(x) + width(y) plusOrMinus 1e-6)
            }
        }
        test("two width-1 intervals multiply to different widths, so width is not a function of widths alone") {
            val product1 = mulInterval(makeInterval(1.0, 3.0), makeInterval(1.0, 3.0))
            val product2 = mulInterval(makeInterval(10.0, 12.0), makeInterval(10.0, 12.0))
            width(makeInterval(1.0, 3.0)) shouldBe width(makeInterval(10.0, 12.0))
            width(product1) shouldBe 4.0
            width(product2) shouldBe 22.0
        }
        test("ex_2_09 matches the two product widths, 4 and 22") {
            ex_2_09() shouldBe (4.0 to 22.0)
        }
    })
