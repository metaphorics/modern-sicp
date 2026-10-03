// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.15

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.structurallyEqual

public class E3_15Test :
    FunSpec({
        test("one stored pair makes the mutation visible through both aliases") {
            val shared = datumList(Symbol("a"), Symbol("b")) as PairCell
            val z1 = pair(shared, shared)

            (z1.first === z1.second) shouldBe true
            val result = setToWow(z1)

            (result === z1) shouldBe true
            (z1.first === z1.second) shouldBe true
            structurallyEqual(
                z1,
                pair(datumList(Symbol("wow"), Symbol("b")), datumList(Symbol("wow"), Symbol("b"))),
            ) shouldBe true
        }

        test("equal but independent pairs do not share the mutation") {
            val left = datumList(Symbol("a"), Symbol("b"))
            val right = datumList(Symbol("a"), Symbol("b"))
            val z2 = pair(left, right)

            (z2.first === z2.second) shouldBe false
            val result = setToWow(z2)

            (result === z2) shouldBe true
            (z2.first === left) shouldBe true
            (z2.second === right) shouldBe true
            structurallyEqual(z2.first, datumList(Symbol("wow"), Symbol("b"))) shouldBe true
            structurallyEqual(z2.second, datumList(Symbol("a"), Symbol("b"))) shouldBe true
        }
    })
