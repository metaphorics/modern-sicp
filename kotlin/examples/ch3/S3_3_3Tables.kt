// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.3.3, representing tables

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.structurallyEqual

/** A one-dimensional table backed by a linked chain of host `PairCell` records.
 * The header's first field is the `*table*` marker; its second points to records. */
public class Table(
    /** The host-data equality test for keys; exercise 3.24 supplies another one. */
    public val sameKey: (Datum, Datum) -> Boolean = ::structurallyEqual,
) {
    private val header: PairCell = pair(Symbol("*table*"), Empty)

    /** Find the record whose key matches, or return null when no record matches. */
    public fun findRecord(
        key: Datum,
        records: Datum,
    ): PairCell? {
        if (records !is PairCell) {
            return null
        }
        val record = records.first
        return if (record is PairCell && sameKey(key, record.first)) {
            record
        } else {
            findRecord(key, records.second)
        }
    }

    /** Return the datum under [key], or null when it is absent. */
    public fun lookup(key: Datum): Datum? = findRecord(key, header.second)?.second

    /** Insert or replace a datum under [key]. */
    public fun insert(
        key: Datum,
        value: Datum,
    ) {
        val record = findRecord(key, header.second)
        if (record == null) {
            header.second = pair(pair(key, value), header.second)
            return
        }
        record.second = value
    }

    /** The book's two-dimensional lookup: key1 names a subtable on the
     * backbone, key2 a record within it. */
    public fun lookup2d(
        key1: Datum,
        key2: Datum,
    ): Datum? {
        val subtable = findRecord(key1, header.second) ?: return null
        return findRecord(key2, subtable.second)?.second
    }

    /** Insert or replace a datum under a pair of keys. */
    public fun insert2d(
        key1: Datum,
        key2: Datum,
        value: Datum,
    ) {
        val subtable = findRecord(key1, header.second)
        if (subtable == null) {
            header.second = pair(pair(key1, datumList(pair(key2, value))), header.second)
            return
        }
        val record = findRecord(key2, subtable.second)
        if (record != null) {
            record.second = value
            return
        }
        subtable.second = pair(pair(key2, value), subtable.second)
    }
}

/** Build the table used in the examples. */
public fun makeTable(): Table = Table()

public class S3_3_3TablesTest :
    FunSpec({
        test("the headed table of figure 3.22: a: 1, b: 2, c: 3") {
            val table = makeTable()
            table.insert(Symbol("a"), Whole(1))
            table.insert(Symbol("b"), Whole(2))
            table.insert(Symbol("c"), Whole(3))
            table.lookup(Symbol("a")) shouldBe Whole(1)
            table.lookup(Symbol("b")) shouldBe Whole(2)
            table.lookup(Symbol("c")) shouldBe Whole(3)
        }

        test("lookup of a missing key is null") {
            val table = makeTable()
            table.insert(Symbol("a"), Whole(1))
            table.lookup(Symbol("z")).shouldBeNull()
        }

        test("inserting under an existing key overwrites the record in place") {
            val table = makeTable()
            table.insert(Symbol("b"), Whole(2))
            table.insert(Symbol("b"), Whole(20))
            table.lookup(Symbol("b")) shouldBe Whole(20)
        }

        test("the figure 3.23 two-dimensional table") {
            val table = makeTable()
            table.insert2d(Symbol("letters"), Symbol("a"), Whole(97))
            table.insert2d(Symbol("letters"), Symbol("b"), Whole(98))
            table.insert2d(Symbol("math"), Symbol("+"), Whole(43))
            table.insert2d(Symbol("math"), Symbol("-"), Whole(45))
            table.insert2d(Symbol("math"), Symbol("*"), Whole(42))
            table.lookup2d(Symbol("letters"), Symbol("b")) shouldBe Whole(98)
            table.lookup2d(Symbol("math"), Symbol("*")) shouldBe Whole(42)
            table.lookup2d(Symbol("math"), Symbol("/")).shouldBeNull()
        }

        test("a two-dimensional insert under an existing pair of keys overwrites") {
            val table = makeTable()
            table.insert2d(Symbol("math"), Symbol("+"), Whole(43))
            table.insert2d(Symbol("math"), Symbol("+"), Whole(99))
            table.lookup2d(Symbol("math"), Symbol("+")) shouldBe Whole(99)
        }

        test("get and put over one local operation table") {
            val operationTable = makeTable()
            val get = { key1: Datum, key2: Datum -> operationTable.lookup2d(key1, key2) }
            val put = { key1: Datum, key2: Datum, value: Datum ->
                operationTable.insert2d(key1, key2, value)
            }
            put(Symbol("real-part"), Symbol("rectangular"), Whole(0))
            get(Symbol("real-part"), Symbol("rectangular")) shouldBe Whole(0)
            get(Symbol("real-part"), Symbol("polar")).shouldBeNull()
        }
    })
