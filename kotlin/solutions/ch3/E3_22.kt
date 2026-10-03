// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.22

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.pair

/** A message-passing queue implemented by an object over two captured pointers. */
public interface MessagePassingQueue {
    public fun emptyQueue(): Boolean

    public fun insert(item: Datum)

    public fun delete(): Either<QueueError, Datum>

    public fun items(): List<Datum>
}

/**
 * The two pointers live in local variables of this call. The returned
 * object's methods capture them, so separate calls build independent queues.
 */
public fun makeQueue(): MessagePassingQueue {
    var front: PairCell? = null
    var rear: PairCell? = null
    return object : MessagePassingQueue {
        override fun emptyQueue(): Boolean = front == null

        override fun insert(item: Datum) {
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

        override fun delete(): Either<QueueError, Datum> {
            val head = front
            if (head == null) {
                return Either.Left(QueueError.EmptyQueue)
            }
            front = head.second as? PairCell
            if (front == null) {
                rear = null
            }
            return Either.Right(head.first)
        }

        override fun items(): List<Datum> {
            val result = mutableListOf<Datum>()
            var cursor = front
            while (cursor != null) {
                result.add(cursor.first)
                cursor = cursor.second as? PairCell
            }
            return result
        }
    }
}
