// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.3.1

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Datum
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.Truth
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.structurallyEqual

/**
 * Search a datum sequence with Kotlin equality. Atomic values compare by
 * content, while mutable pair cells retain identity; a hit returns the
 * original suffix, and a miss is an explicit native Boolean datum.
 */
public fun findDatumTail(
    item: Datum,
    sequence: Datum,
): Datum =
    when (sequence) {
        is PairCell -> {
            if (sequence.first == item) {
                sequence
            } else {
                findDatumTail(item, sequence.second)
            }
        }

        else -> {
            Truth(false)
        }
    }

public class S2_3_1QuotationTest :
    FunSpec({
        test("datumList retains numbers in a native pair chain") {
            val sequence = datumList(Whole(1L), Whole(2L))
            val first = sequence as PairCell
            first.first shouldBe Whole(1L)
            (first.second as PairCell).first shouldBe Whole(2L)
        }
        test("native rendering names symbols and pair fields explicitly") {
            datumList(Symbol("a"), Symbol("b")).toString() shouldBe
                "PairCell(first=Symbol(name=\"a\"), second=PairCell(first=Symbol(name=\"b\"), second=Empty))"
        }
        test("symbol values can be nested as ordinary data") {
            val nested = datumList(Symbol("outer"), datumList(Symbol("inner"), Whole(3L)))
            val first = nested as PairCell
            first.first shouldBe Symbol("outer")
            structurallyEqual(first.second, datumList(datumList(Symbol("inner"), Whole(3L)))) shouldBe true
        }
        test("the membership search returns an explicit false datum when absent") {
            val sequence = datumList(Symbol("pear"), Symbol("banana"), Symbol("prune"))
            findDatumTail(Symbol("apple"), sequence) shouldBe Truth(false)
        }
        test("the membership search returns the original suffix at the first top-level match") {
            val sequence =
                datumList(
                    Symbol("x"),
                    datumList(Symbol("apple"), Symbol("sauce")),
                    Symbol("y"),
                    Symbol("apple"),
                    Symbol("pear"),
                )
            val afterX = (sequence as PairCell).second as PairCell
            val afterNested = afterX.second as PairCell
            val afterY = afterNested.second as PairCell
            val expectedSuffix = afterY
            (findDatumTail(Symbol("apple"), sequence) === expectedSuffix) shouldBe true
        }
        test("membership compares mutable pair values by identity") {
            val pair = datumList(Symbol("apple"), Symbol("sauce"))
            val sequence = datumList(pair)
            (findDatumTail(pair, sequence) === sequence) shouldBe true
            findDatumTail(datumList(Symbol("apple"), Symbol("sauce")), sequence) shouldBe Truth(false)
        }
    })
