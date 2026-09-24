// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.2.1

package sicp.ch2.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.vlist

/**
 * The sequence 1, 2, 3, 4 as a chain of pairs, exactly the book's
 * box-and-pointer picture: `cons(1, cons(2, cons(3, cons(4, VNil))))`.
 * `vlist` builds the same chain from a flat argument list, and its
 * `toString` prints the book's own `(1 2 3 4)`.
 */
public fun oneThroughFourByHand(): Value = cons(VInt(1L), cons(VInt(2L), cons(VInt(3L), cons(VInt(4L), VNil))))

public fun oneThroughFourByVlist(): Value = vlist(VInt(1L), VInt(2L), VInt(3L), VInt(4L))

private fun carOf(v: Value): Value = (v as VPair).car

private fun cdrOf(v: Value): Value = (v as VPair).cdr

private fun numOf(v: Value): Long = (v as VInt).n

/**
 * The book's `list-ref`, hand-rolled by walking `n` `cdr`s and taking the
 * `car`: this is the chain-of-pairs representation the section opens
 * with. Everyday Kotlin work represents a sequence as `List<T>`, whose
 * indexing does the same walk under `get`; 2.2.3 switches to that
 * representation for the sequence-operation pipelines.
 */
public fun listRef(
    items: Value,
    n: Long,
): Value = if (n == 0L) carOf(items) else listRef(cdrOf(items), n - 1L)

/** The book's `length`, recursive over the chain. */
public fun lengthList(items: Value): Long = if (items is VNil) 0L else 1L + lengthList(cdrOf(items))

/** The book's `append`, recursive over the first chain. */
public fun appendList(
    list1: Value,
    list2: Value,
): Value = if (list1 is VNil) list2 else cons(carOf(list1), appendList(cdrOf(list1), list2))

/** The book's `scale-list`, multiplying every element by `factor`. */
public fun scaleList(
    items: Value,
    factor: Long,
): Value =
    if (items is VNil) {
        VNil
    } else {
        cons(VInt(numOf(carOf(items)) * factor), scaleList(cdrOf(items), factor))
    }

/**
 * The book's `map`: applies [f] to every element, building a new chain.
 * `scaleListViaMap` redefines `scale-list` in terms of it, the book's own
 * next step.
 */
public fun mapList(
    f: (Value) -> Value,
    items: Value,
): Value = if (items is VNil) VNil else cons(f(carOf(items)), mapList(f, cdrOf(items)))

public fun scaleListViaMap(
    items: Value,
    factor: Long,
): Value = mapList({ v -> VInt(numOf(v) * factor) }, items)

/** The book's `for-each`: applies [action] to every element, for its side effects, returning nothing useful. */
public fun forEachValue(
    items: Value,
    action: (Value) -> Unit,
) {
    var cursor = items
    while (cursor is VPair) {
        action(cursor.car)
        cursor = cursor.cdr
    }
}

public class S2_2_1RepresentingSequencesTest :
    FunSpec({
        test("cons chains and vlist build the same sequence, and print the book's own surface syntax") {
            oneThroughFourByHand().toString() shouldBe "(1 2 3 4)"
            oneThroughFourByVlist().toString() shouldBe "(1 2 3 4)"
        }
        test("listRef walks n cdrs then takes a car") {
            val squares = vlist(VInt(1L), VInt(4L), VInt(9L), VInt(16L), VInt(25L))
            listRef(squares, 0L) shouldBe VInt(1L)
            listRef(squares, 3L) shouldBe VInt(16L)
        }
        test("lengthList counts the empty-list-terminated chain") {
            lengthList(vlist(VInt(1L), VInt(3L), VInt(5L))) shouldBe 3L
            lengthList(VNil) shouldBe 0L
        }
        test("appendList glues the first chain's elements onto the second") {
            appendList(vlist(VInt(1L), VInt(2L)), vlist(VInt(3L), VInt(4L))).toString() shouldBe "(1 2 3 4)"
        }
        test("scaleList multiplies every element by the factor: the book's (10 20 30 40 50)") {
            val list = vlist(VInt(1L), VInt(2L), VInt(3L), VInt(4L), VInt(5L))
            scaleList(list, 10L).toString() shouldBe "(10 20 30 40 50)"
        }
        test("scaleListViaMap agrees with the hand-rolled scaleList") {
            val list = vlist(VInt(1L), VInt(2L), VInt(3L), VInt(4L), VInt(5L))
            scaleListViaMap(list, 10L).toString() shouldBe scaleList(list, 10L).toString()
        }
        test("forEachValue visits every element in order, for its side effects") {
            val seen = mutableListOf<Long>()
            forEachValue(vlist(VInt(57L), VInt(321L), VInt(88L))) { v -> seen.add(numOf(v)) }
            seen shouldBe listOf(57L, 321L, 88L)
        }
    })
