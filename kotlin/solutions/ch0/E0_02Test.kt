// SPDX-License-Identifier: GPL-3.0-only
// Chapter 0 primer

package sicp.ch0.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.int
import io.kotest.property.arbitrary.long
import io.kotest.property.checkAll

private val inc: (Long) -> Long = { x -> x + 1L }
private val double: (Long) -> Long = { x -> x * 2L }

public class E0_02Test :
    FunSpec({
        test("hand-computed cases") {
            compose(inc, double)(5L) shouldBe 11L
            compose(double, inc)(5L) shouldBe 12L
            repeated(inc, 10)(0L) shouldBe 10L
            repeated(double, 3)(1L) shouldBe 8L
            repeated(inc, 0)(7L) shouldBe 7L
        }
        test("compose agrees with f(g(x)) for every generated input") {
            checkAll(Arb.long(-1000L, 1000L)) { x ->
                compose(inc, double)(x) shouldBe inc(double(x))
            }
        }
        test("repeated(inc, n) agrees with n increments") {
            checkAll(Arb.long(-100L, 100L), Arb.int(0, 10)) { x, n ->
                repeated(inc, n)(x) shouldBe x + n
            }
        }
        test("repeated composes: two doublings after three increments") {
            checkAll(Arb.long(-100L, 100L)) { x ->
                compose(repeated(double, 2), repeated(inc, 3))(x) shouldBe 4L * (x + 3L)
            }
        }
    })
