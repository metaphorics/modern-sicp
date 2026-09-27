// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, section 3.3.2, representing queues

package sicp.ch3.examples

import arrow.core.Either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.setCdr

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

public class S3_3_2QueuesTest :
    FunSpec({
        test("the figure 3.18 session: insert a, insert b, delete, insert c and d, delete") {
            val q = Queue()
            q.insert(VSym("a"))
            q.printQueue() shouldBe "(a)"
            q.insert(VSym("b"))
            q.printQueue() shouldBe "(a b)"
            q.delete() shouldBe Either.Right(VSym("a"))
            q.printQueue() shouldBe "(b)"
            q.insert(VSym("c"))
            q.insert(VSym("d"))
            q.printQueue() shouldBe "(b c d)"
            q.delete() shouldBe Either.Right(VSym("b"))
            q.printQueue() shouldBe "(c d)"
        }

        test("a fresh queue is empty and prints ()") {
            val q = Queue()
            q.emptyQueue() shouldBe true
            q.printQueue() shouldBe "()"
            q.frontCell().shouldBeNull()
            q.rearCell().shouldBeNull()
        }

        test("delete on an empty queue is Left(EmptyQueue)") {
            val q = Queue()
            q.delete() shouldBe Either.Left(QueueError.EmptyQueue)
        }

        test("the queue is reusable after emptying") {
            val q = Queue()
            q.insert(VSym("a"))
            q.delete() shouldBe Either.Right(VSym("a"))
            q.emptyQueue() shouldBe true
            q.insert(VSym("b"))
            q.delete() shouldBe Either.Right(VSym("b"))
            q.emptyQueue() shouldBe true
        }

        test("the rear cell is the last cell of the front chain (3.21's hidden pair)") {
            val q = Queue()
            q.insert(VSym("a"))
            q.insert(VSym("b"))
            q.frontCell()?.toString() shouldBe "(a b)"
            q.rearCell()?.car shouldBe VSym("b")
            q.delete() shouldBe Either.Right(VSym("a"))
            q.rearCell()?.car shouldBe VSym("b")
            q.delete() shouldBe Either.Right(VSym("b"))
            q.rearCell().shouldBeNull()
        }
    })
