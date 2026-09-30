// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.2.1

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.structurallyEqual

/**
 * Proper sequences as mutable pair cells terminated by [Empty]. The hand-built
 * chain and [datumList] use the same native data representation.
 */
public fun oneThroughFourByHand(): Datum = pair(Whole(1L), pair(Whole(2L), pair(Whole(3L), pair(Whole(4L), Empty))))

public fun oneThroughFourByDatumList(): Datum = datumList(Whole(1L), Whole(2L), Whole(3L), Whole(4L))

private fun firstOf(value: Datum): Datum = (value as PairCell).first

private fun restOf(value: Datum): Datum = (value as PairCell).second

private fun numberOf(value: Datum): Long = (value as Whole).value

/**
 * Walks a pair chain to select the item at [index]. Native Kotlin sequences
 * use `List<T>` and indexing; this exercise makes the pair-by-pair traversal
 * explicit.
 */
public fun listRef(
    items: Datum,
    index: Long,
): Datum = if (index == 0L) firstOf(items) else listRef(restOf(items), index - 1L)

/** Count elements recursively until the proper-list terminator. */
public fun lengthList(items: Datum): Long = if (items === Empty) 0L else 1L + lengthList(restOf(items))

/** Copy the first chain onto the second chain. */
public fun appendList(
    list1: Datum,
    list2: Datum,
): Datum = if (list1 === Empty) list2 else pair(firstOf(list1), appendList(restOf(list1), list2))

/** Multiply every whole-number element by [factor]. */
public fun scaleList(
    items: Datum,
    factor: Long,
): Datum =
    if (items === Empty) {
        Empty
    } else {
        pair(Whole(numberOf(firstOf(items)) * factor), scaleList(restOf(items), factor))
    }

/** Apply [f] to every element and build a new pair chain. */
public fun mapList(
    f: (Datum) -> Datum,
    items: Datum,
): Datum = if (items === Empty) Empty else pair(f(firstOf(items)), mapList(f, restOf(items)))

public fun scaleListViaMap(
    items: Datum,
    factor: Long,
): Datum = mapList({ value -> Whole(numberOf(value) * factor) }, items)

/** Visit every element in order for its side effects. */
public fun forEachValue(
    items: Datum,
    action: (Datum) -> Unit,
) {
    var cursor = items
    while (cursor is PairCell) {
        action(cursor.first)
        cursor = cursor.second
    }
}

public class S2_2_1RepresentingSequencesTest :
    FunSpec({
        test("hand-built and variadic construction preserve the whole proper sequence") {
            structurallyEqual(oneThroughFourByHand(), oneThroughFourByDatumList()) shouldBe true
            structurallyEqual(
                oneThroughFourByDatumList(),
                datumList(Whole(1L), Whole(2L), Whole(3L), Whole(4L)),
            ) shouldBe true
        }
        test("native rendering exposes the pair-cell structure") {
            oneThroughFourByHand().toString() shouldBe
                "PairCell(first=Whole(value=1), second=PairCell(first=Whole(value=2), second=PairCell(first=Whole(value=3), second=PairCell(first=Whole(value=4), second=Empty))))"
        }
        test("listRef follows pair links to the requested element") {
            val squares = datumList(Whole(1L), Whole(4L), Whole(9L), Whole(16L), Whole(25L))
            listRef(squares, 0L) shouldBe Whole(1L)
            listRef(squares, 3L) shouldBe Whole(16L)
        }
        test("lengthList counts the Empty-terminated chain") {
            lengthList(datumList(Whole(1L), Whole(3L), Whole(5L))) shouldBe 3L
            lengthList(Empty) shouldBe 0L
        }
        test("appendList keeps the first chain's order before the second") {
            val result = appendList(datumList(Whole(1L), Whole(2L)), datumList(Whole(3L), Whole(4L)))
            structurallyEqual(result, datumList(Whole(1L), Whole(2L), Whole(3L), Whole(4L))) shouldBe true
        }
        test("scaleList multiplies every whole-number element") {
            val list = datumList(Whole(1L), Whole(2L), Whole(3L), Whole(4L), Whole(5L))
            structurallyEqual(
                scaleList(list, 10L),
                datumList(Whole(10L), Whole(20L), Whole(30L), Whole(40L), Whole(50L)),
            ) shouldBe true
        }
        test("scaleListViaMap agrees structurally with the hand-rolled traversal") {
            val list = datumList(Whole(1L), Whole(2L), Whole(3L), Whole(4L), Whole(5L))
            structurallyEqual(scaleListViaMap(list, 10L), scaleList(list, 10L)) shouldBe true
        }
        test("forEachValue visits every element in order") {
            val seen = mutableListOf<Long>()
            forEachValue(datumList(Whole(57L), Whole(321L), Whole(88L))) { value -> seen.add(numberOf(value)) }
            seen shouldBe listOf(57L, 321L, 88L)
        }
    })
