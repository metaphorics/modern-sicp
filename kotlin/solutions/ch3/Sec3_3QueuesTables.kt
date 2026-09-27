// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.3, shared queue and table code for the 3.3.2/3.3.3 exercises

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.cons
import sicp.runtime.equalv
import sicp.runtime.setCdr
import sicp.runtime.vlist

public sealed interface QueueError {
    public data object EmptyQueue : QueueError
}

/** The book's queue of 3.3.2: two mutable pointers into one chain of
 * mutable pairs, plus the operations insert-queue!, delete-queue!, and
 * print-queue as methods. */
public class Queue {
    private var front: VPair? = null
    private var rear: VPair? = null

    /** Whether the queue holds no items. */
    public fun emptyQueue(): Boolean = front == null

    /** The front pointer cell; null exactly when the queue is empty.
     * Exercise 3.21 observes the hidden pair through this. */
    public fun frontCell(): VPair? = front

    /** The rear pointer cell; null exactly when the queue is empty. */
    public fun rearCell(): VPair? = rear

    /** The book's insert-queue!: hang the new cell off the rear pointer. */
    public fun insert(item: Value) {
        val cell = VPair(item, VNil)
        val tail = rear
        if (tail == null) {
            front = cell
            rear = cell
        } else {
            tail.setCdr(cell)
            rear = cell
        }
    }

    /** The book's delete-queue!: advance the front pointer; Left of
     * EmptyQueue when the queue is empty, as the book's error was. */
    public fun delete(): Either<QueueError, Value> {
        val head = front
        return if (head == null) {
            Either.Left(QueueError.EmptyQueue)
        } else {
            front = head.cdr as? VPair
            if (front == null) {
                rear = null
            }
            Either.Right(head.car)
        }
    }

    /** The book's print-queue: the front chain, rendered like the book's list. */
    public fun printQueue(): String = front?.toString() ?: "()"
}

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
