// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.24

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.VSym

public class E3_24Test :
    FunSpec({
        test("a tolerance key test finds a key near the probe") {
            val near =
                makeTable { a, b -> a is VInt && b is VInt && kotlin.math.abs(a.n - b.n) <= 5 }
            near.insert(VInt(40), VSym("fortyish"))
            near.lookup(VInt(41)) shouldBe VSym("fortyish")
            near.lookup(VInt(52)).shouldBeNull()
        }

        test("with a tolerance, near keys are the same slot: the second insert overwrites the first") {
            val near =
                makeTable { a, b -> a is VInt && b is VInt && kotlin.math.abs(a.n - b.n) <= 5 }
            near.insert(VInt(40), VInt(1))
            near.insert(VInt(43), VInt(2))
            near.lookup(VInt(40)) shouldBe VInt(2)
            near.lookup(VInt(43)) shouldBe VInt(2)
        }

        test("the default table still wants the exact key") {
            val exact = makeTable()
            exact.insert(VInt(40), VSym("x"))
            exact.lookup(VInt(41)).shouldBeNull()
            exact.lookup(VInt(40)) shouldBe VSym("x")
        }

        test("the caller's predicate runs on symbols too") {
            val caseFold =
                makeTable { a, b -> a is VSym && b is VSym && a.name.lowercase() == b.name.lowercase() }
            caseFold.insert(VSym("Car"), VSym("auto"))
            caseFold.lookup(VSym("car")) shouldBe VSym("auto")
        }
    })
