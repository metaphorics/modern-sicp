// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.26

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.VSym

public class E3_26Test :
    FunSpec({
        test("every inserted key is found by a root-to-leaf walk") {
            val t = TreeTable()
            t.insert(VSym("d"), VInt(4))
            t.insert(VSym("b"), VInt(2))
            t.insert(VSym("f"), VInt(6))
            t.insert(VSym("a"), VInt(1))
            t.insert(VSym("e"), VInt(5))
            t.lookup(VSym("a")) shouldBe VInt(1)
            t.lookup(VSym("b")) shouldBe VInt(2)
            t.lookup(VSym("d")) shouldBe VInt(4)
            t.lookup(VSym("e")) shouldBe VInt(5)
            t.lookup(VSym("f")) shouldBe VInt(6)
        }

        test("a key outside the stored range walks to a null leaf") {
            val t = TreeTable()
            t.insert(VSym("m"), VInt(13))
            t.lookup(VSym("a")).shouldBeNull()
            t.lookup(VSym("z")).shouldBeNull()
        }

        test("inserting an existing key overwrites the record in place") {
            val t = TreeTable()
            t.insert(VSym("b"), VInt(2))
            t.insert(VSym("a"), VInt(1))
            t.insert(VSym("b"), VInt(20))
            t.lookup(VSym("b")) shouldBe VInt(20)
            t.lookup(VSym("a")) shouldBe VInt(1)
        }

        test("integer keys order numerically") {
            val t = TreeTable()
            t.insert(VInt(10), VSym("ten"))
            t.insert(VInt(2), VSym("two"))
            t.insert(VInt(33), VSym("thirty-three"))
            t.lookup(VInt(2)) shouldBe VSym("two")
            t.lookup(VInt(10)) shouldBe VSym("ten")
            t.lookup(VInt(33)) shouldBe VSym("thirty-three")
            t.lookup(VInt(17)).shouldBeNull()
        }

        test("the records are shared mutable nodes: a value change is visible through the table") {
            val t = TreeTable()
            t.insert(VSym("k"), VInt(1))
            t.insert(VSym("j"), VInt(0))
            t.lookup(VSym("k")) shouldBe VInt(1)
            t.insert(VSym("k"), VInt(2))
            t.lookup(VSym("k")) shouldBe VInt(2)
        }
    })
