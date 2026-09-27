// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.11

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.double
import io.kotest.property.arbitrary.filter
import io.kotest.property.checkAll

public class E2_11Test :
    FunSpec({
        test("mulIntervalCases agrees with mulInterval across every sign combination") {
            checkAll(
                Arb.double(-10.0..10.0).filter { it.isFinite() },
                Arb.double(0.0..10.0).filter { it.isFinite() },
                Arb.double(-10.0..10.0).filter { it.isFinite() },
                Arb.double(0.0..10.0).filter { it.isFinite() },
            ) { cx, wx, cy, wy ->
                val x = makeInterval(cx - wx, cx + wx)
                val y = makeInterval(cy - wy, cy + wy)
                mulIntervalCases(x, y) shouldBe mulInterval(x, y)
            }
        }
        test("both intervals spanning zero needs all four corner products") {
            mulIntervalCases(makeInterval(-2.0, 3.0), makeInterval(-1.0, 2.0)) shouldBe Interval(-4.0, 6.0)
        }
        test("ex_2_11 matches the same both-span-zero case") {
            ex_2_11() shouldBe Interval(-4.0, 6.0)
        }
    })
