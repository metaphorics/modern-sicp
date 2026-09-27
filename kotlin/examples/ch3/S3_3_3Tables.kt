// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.3.3, representing tables

package sicp.ch3.examples

import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.equalv
import sicp.runtime.setCdr
import sicp.runtime.vlist

/** The book's one-dimensional table of 3.3.3: a headed list of records
 * over the mutable pair. The first backbone pair is the object that
 * represents the table itself: its car holds the dummy `*table*` marker
 * (a subtable carries its own key there), its cdr the chain of records. */
public class Table(
    /** The book's ``equality'' test for keys; the default is `equal?`,
     * and exercise 3.24 hands the constructor a different one. */
    public val sameKey: (Value, Value) -> Boolean = ::equalv,
) {
    private val header: VPair = VPair(VSym("*table*"), VNil)

    /** The book's assoc: the first record whose key passes the table's
     * key test, or null. The scan runs over the records it is handed,
     * so it never sees the dummy record. */
    public fun assoc(
        key: Value,
        records: Value,
    ): VPair? {
        if (records !is VPair) {
            return null
        }
        val record = records.car
        return if (record is VPair && sameKey(key, record.car)) {
            record
        } else {
            assoc(key, records.cdr)
        }
    }

    /** The book's lookup: the value stored under key, or null (the
     * book's false). */
    public fun lookup(key: Value): Value? = assoc(key, header.cdr)?.cdr

    /** The book's insert!: an existing record gets the new value in
     * place; a fresh record is spliced in right after the header, the
     * fixed location the header exists to provide. */
    public fun insert(
        key: Value,
        value: Value,
    ) {
        val record = assoc(key, header.cdr)
        if (record == null) {
            header.setCdr(cons(VPair(key, value), header.cdr))
            return
        }
        record.setCdr(value)
    }

    /** The book's two-dimensional lookup: key1 names a subtable on the
     * backbone, key2 a record within it. */
    public fun lookup2d(
        key1: Value,
        key2: Value,
    ): Value? {
        val subtable = assoc(key1, header.cdr) ?: return null
        return assoc(key2, subtable.cdr)?.cdr
    }

    /** The book's two-dimensional insert!: a subtable is a headed list
     * whose header carries key1 where the top table carries `*table*`. */
    public fun insert2d(
        key1: Value,
        key2: Value,
        value: Value,
    ) {
        val subtable = assoc(key1, header.cdr)
        if (subtable == null) {
            header.setCdr(
                cons(VPair(key1, vlist(VPair(key2, value))), header.cdr),
            )
            return
        }
        val record = assoc(key2, subtable.cdr)
        if (record != null) {
            record.setCdr(value)
            return
        }
        subtable.setCdr(cons(VPair(key2, value), subtable.cdr))
    }
}

/** The book's make-table. */
public fun makeTable(): Table = Table()

public class S3_3_3TablesTest :
    FunSpec({
        test("the headed table of figure 3.22: a: 1, b: 2, c: 3") {
            val table = makeTable()
            table.insert(VSym("a"), VInt(1))
            table.insert(VSym("b"), VInt(2))
            table.insert(VSym("c"), VInt(3))
            table.lookup(VSym("a")) shouldBe VInt(1)
            table.lookup(VSym("b")) shouldBe VInt(2)
            table.lookup(VSym("c")) shouldBe VInt(3)
        }

        test("lookup of a missing key is null, the book's false") {
            val table = makeTable()
            table.insert(VSym("a"), VInt(1))
            table.lookup(VSym("z")).shouldBeNull()
        }

        test("inserting under an existing key overwrites the record in place") {
            val table = makeTable()
            table.insert(VSym("b"), VInt(2))
            table.insert(VSym("b"), VInt(20))
            table.lookup(VSym("b")) shouldBe VInt(20)
        }

        test("the figure 3.23 two-dimensional table") {
            val table = makeTable()
            table.insert2d(VSym("letters"), VSym("a"), VInt(97))
            table.insert2d(VSym("letters"), VSym("b"), VInt(98))
            table.insert2d(VSym("math"), VSym("+"), VInt(43))
            table.insert2d(VSym("math"), VSym("-"), VInt(45))
            table.insert2d(VSym("math"), VSym("*"), VInt(42))
            table.lookup2d(VSym("letters"), VSym("b")) shouldBe VInt(98)
            table.lookup2d(VSym("math"), VSym("*")) shouldBe VInt(42)
            table.lookup2d(VSym("math"), VSym("/")).shouldBeNull()
        }

        test("a two-dimensional insert under an existing pair of keys overwrites") {
            val table = makeTable()
            table.insert2d(VSym("math"), VSym("+"), VInt(43))
            table.insert2d(VSym("math"), VSym("+"), VInt(99))
            table.lookup2d(VSym("math"), VSym("+")) shouldBe VInt(99)
        }

        test("get and put over one local operation table") {
            val operationTable = makeTable()
            val get = { key1: Value, key2: Value -> operationTable.lookup2d(key1, key2) }
            val put = { key1: Value, key2: Value, value: Value ->
                operationTable.insert2d(key1, key2, value)
            }
            put(VSym("real-part"), VSym("rectangular"), VInt(0))
            get(VSym("real-part"), VSym("rectangular")) shouldBe VInt(0)
            get(VSym("real-part"), VSym("polar")).shouldBeNull()
        }
    })
