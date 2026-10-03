// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.12

package sicp.ch3.exercises

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.structurallyEqual

public class E3_12Test :
    FunSpec({
        test("append copies the first spine and shares the second list") {
            val x = datumList(Symbol("a"), Symbol("b")) as PairCell
            val y = datumList(Symbol("c"), Symbol("d")) as PairCell
            val originalTail = x.second

            val z = append(x, y)

            structurallyEqual(z, datumList(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d"))) shouldBe true
            (x.second === originalTail) shouldBe true
            (lastPair(z as PairCell) === lastPair(y)) shouldBe true
        }

        test("appendBang splices the second chain in place and returns x") {
            val x = datumList(Symbol("a"), Symbol("b")) as PairCell
            val y = datumList(Symbol("c"), Symbol("d")) as PairCell
            val bCell = x.second as PairCell

            val result = appendBang(x, y)

            (result === x) shouldBe true
            structurallyEqual(x.second, datumList(Symbol("b"), Symbol("c"), Symbol("d"))) shouldBe true
            (bCell.second === y) shouldBe true
        }

        test("lastPair returns the final pair, itself for a one-pair list") {
            val x = datumList(Symbol("a"), Symbol("b"), Symbol("c")) as PairCell
            val second = x.second as PairCell
            val single = datumList(Symbol("a")) as PairCell

            (lastPair(x) === second.second) shouldBe true
            (lastPair(single) === single) shouldBe true
        }

        test("appendBang accepts an empty-list tail") {
            val x = datumList(Symbol("a"), Symbol("b")) as PairCell
            val originalTail = x.second

            appendBang(x, Empty)

            (x.second === originalTail) shouldBe true
            structurallyEqual(x, datumList(Symbol("a"), Symbol("b"))) shouldBe true
        }
    })
