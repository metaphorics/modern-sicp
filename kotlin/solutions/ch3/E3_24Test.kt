// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.24

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.Symbol
import sicp.runtime.Whole

public class E3_24Test :
    FunSpec({
        test("a tolerance key test finds a key near the probe") {
            val near =
                makeTable { a, b -> a is Whole && b is Whole && kotlin.math.abs(a.value - b.value) <= 5 }
            near.insert(Whole(40), Symbol("fortyish"))
            near.lookup(Whole(41)) shouldBe Symbol("fortyish")
            near.lookup(Whole(52)).shouldBeNull()
        }

        test("with a tolerance, near keys are the same slot: the second insert overwrites the first") {
            val near =
                makeTable { a, b -> a is Whole && b is Whole && kotlin.math.abs(a.value - b.value) <= 5 }
            near.insert(Whole(40), Whole(1))
            near.insert(Whole(43), Whole(2))
            near.lookup(Whole(40)) shouldBe Whole(2)
            near.lookup(Whole(43)) shouldBe Whole(2)
        }

        test("the default table still wants the exact key") {
            val exact = makeTable()
            exact.insert(Whole(40), Symbol("x"))
            exact.lookup(Whole(41)).shouldBeNull()
            exact.lookup(Whole(40)) shouldBe Symbol("x")
        }

        test("the caller's predicate runs on symbols too") {
            val caseFold =
                makeTable { a, b -> a is Symbol && b is Symbol && a.name.lowercase() == b.name.lowercase() }
            caseFold.insert(Symbol("Car"), Symbol("auto"))
            caseFold.lookup(Symbol("car")) shouldBe Symbol("auto")
        }
    })
