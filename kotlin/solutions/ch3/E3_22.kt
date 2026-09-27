// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.22

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.Value
import sicp.runtime.setCdr

/** The queue operations of 3.3.2, as an interface: the members the
 * object expression below captures the two local pointers through. */
public interface MessagePassingQueue {
    public fun emptyQueue(): Boolean

    public fun insert(item: Value)

    public fun delete(): Either<QueueError, Value>

    public fun printQueue(): String
}

/**
 * The book's make-queue as a message-passing object: the two pointers
 * are local `var`s of this very call, and the returned object's
 * members capture them. No class field holds the state; two calls to
 * `makeQueue` open two frames and so build two independent queues.
 */
public fun makeQueue(): MessagePassingQueue {
    var front: VPair? = null
    var rear: VPair? = null
    return object : MessagePassingQueue {
        override fun emptyQueue(): Boolean = front == null

        override fun insert(item: Value) {
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

        override fun delete(): Either<QueueError, Value> {
            val head = front
            if (head == null) {
                return Either.Left(QueueError.EmptyQueue)
            }
            front = head.cdr as? VPair
            if (front == null) {
                rear = null
            }
            return Either.Right(head.car)
        }

        override fun printQueue(): String = front?.toString() ?: "()"
    }
}
