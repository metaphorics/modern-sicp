// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.3, shared queue and table code for the 3.3.2/3.3.3 exercises

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.structurallyEqual

public sealed interface QueueError {
    public data object EmptyQueue : QueueError
}

/** A mutable linked queue over host datum pairs. */
public class Queue {
    private var front: PairCell? = null
    private var rear: PairCell? = null

    public fun emptyQueue(): Boolean = front == null

    public fun frontCell(): PairCell? = front

    public fun rearCell(): PairCell? = rear

    public fun insert(item: Datum) {
        val cell = pair(item, Empty)
        val tail = rear
        if (tail == null) {
            front = cell
            rear = cell
        } else {
            tail.second = cell
            rear = cell
        }
    }

    public fun delete(): Either<QueueError, Datum> {
        val head = front
        return if (head == null) {
            Either.Left(QueueError.EmptyQueue)
        } else {
            front = head.second as? PairCell
            if (front == null) {
                rear = null
            }
            Either.Right(head.first)
        }
    }

    public fun items(): List<Datum> {
        val result = mutableListOf<Datum>()
        val seen = java.util.Collections.newSetFromMap(java.util.IdentityHashMap<PairCell, Boolean>())
        var cursor = front
        while (cursor != null && seen.add(cursor)) {
            result.add(cursor.first)
            cursor = cursor.second as? PairCell
        }
        return result
    }
}

/** A table backed by a headed chain of host datum pairs. */
public class Table(
    public val sameKey: (Datum, Datum) -> Boolean = ::structurallyEqual,
) {
    private val header: PairCell = pair(Symbol("*table*"), Empty)

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

    public fun lookup(key: Datum): Datum? = findRecord(key, header.second)?.second

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

    public fun lookup2d(
        key1: Datum,
        key2: Datum,
    ): Datum? {
        val subtable = findRecord(key1, header.second) ?: return null
        return findRecord(key2, subtable.second)?.second
    }

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

/** Build a table with structural equality as its key comparison. */
public fun makeTable(): Table = Table()
