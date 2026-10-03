// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.26

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.Symbol
import sicp.runtime.Whole

public class E3_26Test :
    FunSpec({
        test("every inserted key is found by a root-to-leaf walk") {
            val t = TreeTable()
            t.insert(Symbol("d"), Whole(4))
            t.insert(Symbol("b"), Whole(2))
            t.insert(Symbol("f"), Whole(6))
            t.insert(Symbol("a"), Whole(1))
            t.insert(Symbol("e"), Whole(5))
            t.lookup(Symbol("a")) shouldBe Whole(1)
            t.lookup(Symbol("b")) shouldBe Whole(2)
            t.lookup(Symbol("d")) shouldBe Whole(4)
            t.lookup(Symbol("e")) shouldBe Whole(5)
            t.lookup(Symbol("f")) shouldBe Whole(6)
        }

        test("a key outside the stored range walks to a null leaf") {
            val t = TreeTable()
            t.insert(Symbol("m"), Whole(13))
            t.lookup(Symbol("a")).shouldBeNull()
            t.lookup(Symbol("z")).shouldBeNull()
        }

        test("inserting an existing key overwrites the record in place") {
            val t = TreeTable()
            t.insert(Symbol("b"), Whole(2))
            t.insert(Symbol("a"), Whole(1))
            t.insert(Symbol("b"), Whole(20))
            t.lookup(Symbol("b")) shouldBe Whole(20)
            t.lookup(Symbol("a")) shouldBe Whole(1)
        }

        test("integer keys order numerically") {
            val t = TreeTable()
            t.insert(Whole(10), Symbol("ten"))
            t.insert(Whole(2), Symbol("two"))
            t.insert(Whole(33), Symbol("thirty-three"))
            t.lookup(Whole(2)) shouldBe Symbol("two")
            t.lookup(Whole(10)) shouldBe Symbol("ten")
            t.lookup(Whole(33)) shouldBe Symbol("thirty-three")
            t.lookup(Whole(17)).shouldBeNull()
        }

        test("the records are shared mutable nodes: a value change is visible through the table") {
            val t = TreeTable()
            t.insert(Symbol("k"), Whole(1))
            t.insert(Symbol("j"), Whole(0))
            t.lookup(Symbol("k")) shouldBe Whole(1)
            t.insert(Symbol("k"), Whole(2))
            t.lookup(Symbol("k")) shouldBe Whole(2)
        }
    })
