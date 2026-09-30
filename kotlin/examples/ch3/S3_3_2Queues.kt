// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.3.2, representing queues

package sicp.ch3.examples

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Symbol
import sicp.runtime.pair

public sealed interface QueueError {
    public data object EmptyQueue : QueueError
}

/** A mutable linked queue over host datum pairs. */
public class Queue {
    private var front: PairCell? = null
    private var rear: PairCell? = null

    /** Whether the queue holds no items. */
    public fun emptyQueue(): Boolean = front == null

    /** The front pointer cell; null exactly when the queue is empty. */
    public fun frontCell(): PairCell? = front

    /** The rear pointer cell; null exactly when the queue is empty. */
    public fun rearCell(): PairCell? = rear

    /** Append one item by linking a fresh cell after the rear pointer. */
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

    /** Remove and return the front datum, or report that the queue is empty. */
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

    /** Return each reachable queue cell once as native host data. */
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

public class S3_3_2QueuesTest :
    FunSpec({
        test("the figure 3.18 queue session preserves front-to-rear order") {
            val q = Queue()
            q.insert(Symbol("a"))
            q.items() shouldBe listOf(Symbol("a"))
            q.insert(Symbol("b"))
            q.items() shouldBe listOf(Symbol("a"), Symbol("b"))
            q.delete() shouldBe Either.Right(Symbol("a"))
            q.items() shouldBe listOf(Symbol("b"))
            q.insert(Symbol("c"))
            q.insert(Symbol("d"))
            q.items() shouldBe listOf(Symbol("b"), Symbol("c"), Symbol("d"))
            q.delete() shouldBe Either.Right(Symbol("b"))
            q.items() shouldBe listOf(Symbol("c"), Symbol("d"))
        }

        test("a fresh queue is empty") {
            val q = Queue()
            q.emptyQueue() shouldBe true
            q.items() shouldBe emptyList<Datum>()
            q.frontCell().shouldBeNull()
            q.rearCell().shouldBeNull()
        }

        test("delete on an empty queue is Left(EmptyQueue)") {
            val q = Queue()
            q.delete() shouldBe Either.Left(QueueError.EmptyQueue)
        }

        test("the queue is reusable after emptying") {
            val q = Queue()
            q.insert(Symbol("a"))
            q.delete() shouldBe Either.Right(Symbol("a"))
            q.emptyQueue() shouldBe true
            q.insert(Symbol("b"))
            q.delete() shouldBe Either.Right(Symbol("b"))
            q.emptyQueue() shouldBe true
        }

        test("items visits each cell once if a mutable link is made cyclic") {
            val q = Queue()
            q.insert(Symbol("a"))
            val head = q.frontCell() ?: error("single-item queue lost its front cell")
            head.second = head

            q.items() shouldBe listOf(Symbol("a"))
        }

        test("the rear cell is the last cell of the front chain") {
            val q = Queue()
            q.insert(Symbol("a"))
            q.insert(Symbol("b"))
            val front = q.frontCell()
            val rear = q.rearCell()

            (front?.second === rear) shouldBe true
            rear?.first shouldBe Symbol("b")
            rear?.second shouldBe Empty
            q.delete() shouldBe Either.Right(Symbol("a"))
            q.rearCell()?.first shouldBe Symbol("b")
            q.delete() shouldBe Either.Right(Symbol("b"))
            q.rearCell().shouldBeNull()
        }
    })
