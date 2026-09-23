// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.5

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.long
import io.kotest.property.checkAll
import java.math.BigInteger

public class E2_05Test :
    FunSpec({
        test("carPow and cdrPow recover exactly the exponents consPow encoded") {
            carPow(consPow(3L, 4L)) shouldBe 3L
            cdrPow(consPow(3L, 4L)) shouldBe 4L
        }
        test("consPow(3, 4) is 2^3 * 3^4, which is 648") {
            consPow(3L, 4L) shouldBe BigInteger.valueOf(648L)
        }
        test("the identity law holds for every generated pair of small nonnegative exponents") {
            checkAll(Arb.long(0L..20L), Arb.long(0L..20L)) { a, b ->
                val z = consPow(a, b)
                carPow(z) shouldBe a
                cdrPow(z) shouldBe b
            }
        }
        test("ex_2_05 matches consPow(3, 4) decoded back to (3, 4)") {
            ex_2_05() shouldBe (3L to 4L)
        }
    })
