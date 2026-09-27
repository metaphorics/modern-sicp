// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.4a

package sicp.ch2.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.long
import io.kotest.property.checkAll

public class E2_04aTest :
    FunSpec({
        test("ex_2_04a confirms the identity law for the pair (3, 4)") {
            ex_2_04a() shouldBe true
        }
        test("the identity law holds for every generated pair of Long values") {
            checkAll(Arb.long(), Arb.long()) { x, y ->
                carFn(consFn(x, y)) shouldBe x
                cdrFn(consFn(x, y)) shouldBe y
            }
        }
    })
