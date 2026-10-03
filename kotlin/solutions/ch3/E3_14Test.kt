// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.14

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.structurallyEqual

public class E3_14Test :
    FunSpec({
        test("the result contains the reversed elements") {
            val v = datumList(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d")) as PairCell

            val w = mystery(v)

            structurallyEqual(v, datumList(Symbol("a"))) shouldBe true
            structurallyEqual(w, datumList(Symbol("d"), Symbol("c"), Symbol("b"), Symbol("a"))) shouldBe true
        }

        test("the reversed links reuse v's original cells") {
            val v = datumList(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d")) as PairCell
            val second = v.second as PairCell
            val third = second.second as PairCell
            val last = third.second as PairCell

            val w = mystery(v)

            (w === last) shouldBe true
            (w.second === third) shouldBe true
            ((w.second as PairCell).second === second) shouldBe true
            (((w.second as PairCell).second as PairCell).second === v) shouldBe true
        }

        test("a one-pair chain is its own reverse") {
            val v = datumList(Symbol("a")) as PairCell

            val w = mystery(v)

            (w === v) shouldBe true
            structurallyEqual(v, datumList(Symbol("a"))) shouldBe true
        }
    })
