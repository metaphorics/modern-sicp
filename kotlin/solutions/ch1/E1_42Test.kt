// SPDX-License-Identifier: GPL-3.0-only
// Chapter 1, exercise 1.42

package sicp.ch1.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.long
import io.kotest.property.checkAll

public class E1_42Test :
    FunSpec({
        test("compose(square, inc)(6) is 49") {
            ex_1_42() shouldBe 49L
        }
        test("compose(f, g)(x) agrees with f(g(x)) for every generated input") {
            checkAll(Arb.long(-1000L, 1000L)) { x ->
                compose({ y: Long -> y * 2L }, { y: Long -> y + 3L })(x) shouldBe (x + 3L) * 2L
            }
        }
    })
