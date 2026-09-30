// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.3.1, mutation

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.structurallyEqual

/** Create a `PairCell` and initialize its mutable fields through assignments. */
public fun makePair(
    x: Datum,
    y: Datum,
): PairCell {
    val newPair = pair(Empty, Empty)
    newPair.first = x
    newPair.second = y
    return newPair
}

/**
 * The book's `append` of 2.2.1: a fresh list built by consing the
 * elements of `x` onto `y`; no pair of `x` is touched.
 */
public fun append(
    x: Datum,
    y: Datum,
): Datum =
    if (x !is PairCell) {
        y
    } else {
        makePair(x.first, append(x.second, y))
    }

/** The last pair in a nonempty proper list. */
public fun lastPair(x: PairCell): PairCell = if (x.second === Empty) x else lastPair(x.second as PairCell)

/** The book's `append!`: point the last pair of `x` at `y` and return `x`. */
public fun appendBang(
    x: PairCell,
    y: Datum,
): PairCell {
    lastPair(x).second = y
    return x
}

/** Close [x] back onto its first pair. */
public fun makeCycle(x: PairCell): PairCell {
    lastPair(x).second = x
    return x
}

/**
 * The book's `mystery` of exercise 3.14: reverse a chain in place, reusing
 * the very pairs of `x`.
 */
public fun mystery(x: PairCell): PairCell {
    fun loop(
        x: Datum,
        y: Datum,
    ): PairCell =
        if (x !is PairCell) {
            y as PairCell
        } else {
            val temp = x.second
            x.second = y
            loop(temp, x)
        }
    return loop(x, Empty)
}

/** Replace the first datum in the nested pair reached from [x]. */
public fun setToWow(x: PairCell): PairCell {
    (x.first as PairCell).first = Symbol("wow")
    return x
}

/** A pair represented by two mutable slots captured in a closure. */
public interface PairProc {
    public fun first(): Datum

    public fun second(): Datum

    public fun setFirst(v: Datum)

    public fun setSecond(v: Datum)
}

public fun proceduralPair(
    x: Datum,
    y: Datum,
): PairProc {
    var firstSlot = x
    var secondSlot = y
    return object : PairProc {
        override fun first(): Datum = firstSlot

        override fun second(): Datum = secondSlot

        override fun setFirst(v: Datum) {
            firstSlot = v
        }

        override fun setSecond(v: Datum) {
            secondSlot = v
        }
    }
}

public class S3_3_1MutationTest :
    FunSpec({
        test("a pair's first slot is an assignable Kotlin property") {
            val x = makePair(datumList(Symbol("a"), Symbol("b")), datumList(Symbol("c"), Symbol("d")))
            val replacement = datumList(Symbol("e"), Symbol("f"))

            x.first = replacement

            (x.first === replacement) shouldBe true
            structurallyEqual(x.second, datumList(Symbol("c"), Symbol("d"))) shouldBe true
        }

        test("a pair's second slot is an assignable Kotlin property") {
            val x = makePair(datumList(Symbol("a"), Symbol("b")), datumList(Symbol("c"), Symbol("d")))
            val replacement = datumList(Symbol("e"), Symbol("f"))

            x.second = replacement

            structurallyEqual(x.first, datumList(Symbol("a"), Symbol("b"))) shouldBe true
            (x.second === replacement) shouldBe true
        }

        test("append copies the first list's spine while appendBang splices the second") {
            val x = datumList(Symbol("a"), Symbol("b")) as PairCell
            val y = datumList(Symbol("c"), Symbol("d")) as PairCell
            val originalTail = x.second as PairCell

            val z = append(x, y)

            structurallyEqual(z, datumList(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d"))) shouldBe true
            (x.second === originalTail) shouldBe true

            val w = appendBang(x, y)

            (w === x) shouldBe true
            structurallyEqual(x.second, datumList(Symbol("b"), Symbol("c"), Symbol("d"))) shouldBe true
            (originalTail.second === y) shouldBe true
        }

        test("makeCycle closes the chain back onto its first pair") {
            val z = makeCycle(datumList(Symbol("a"), Symbol("b"), Symbol("c")) as PairCell)
            val second = z.second as PairCell
            val third = second.second as PairCell

            (third.second === z) shouldBe true
            (second === z) shouldBe false
        }

        test("mystery reverses the same pair cells in place") {
            val v = datumList(Symbol("a"), Symbol("b"), Symbol("c"), Symbol("d")) as PairCell
            val second = v.second as PairCell
            val third = second.second as PairCell
            val last = third.second as PairCell

            val w = mystery(v)

            structurallyEqual(v, datumList(Symbol("a"))) shouldBe true
            structurallyEqual(w, datumList(Symbol("d"), Symbol("c"), Symbol("b"), Symbol("a"))) shouldBe true
            (w === last) shouldBe true
            (w.second === third) shouldBe true
        }

        test("mutation through one pair reaches both aliases but not a distinct equal pair") {
            val shared = datumList(Symbol("a"), Symbol("b")) as PairCell
            val z1 = makePair(shared, shared)
            val z2 = makePair(datumList(Symbol("a"), Symbol("b")), datumList(Symbol("a"), Symbol("b")))

            (z1.first === z1.second) shouldBe true
            (z2.first === z2.second) shouldBe false
            structurallyEqual(z1, z2) shouldBe true

            setToWow(z1)
            setToWow(z2)

            structurallyEqual(
                z1,
                makePair(datumList(Symbol("wow"), Symbol("b")), datumList(Symbol("wow"), Symbol("b"))),
            ) shouldBe true
            structurallyEqual(
                z2,
                makePair(datumList(Symbol("wow"), Symbol("b")), datumList(Symbol("a"), Symbol("b"))),
            ) shouldBe true
        }

        test("proceduralPair shares its captured mutable slots through aliases") {
            val p = proceduralPair(Whole(1L), Whole(2L))
            p.first() shouldBe Whole(1L)
            p.second() shouldBe Whole(2L)

            val alias = p
            alias.setFirst(Whole(17L))
            p.first() shouldBe Whole(17L)
        }
    })
