// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.23

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.Value

public sealed interface DequeError {
    public data object EmptyDeque : DequeError
}

/**
 * One cell of the deque's doubly linked chain: the item, the cell
 * behind it, and the cell ahead of it. The extra backward slot is what
 * the plain mutable pair of the section lacks; with it, both ends of
 * the deque are one pointer away from the class's two end pointers.
 */
public class DPair(
    public var item: Value,
    public var prev: DPair?,
    public var next: DPair?,
)

/**
 * The book's deque of 3.3.2's exercise: make-deque, empty-deque?,
 * front-deque, rear-deque, and the four mutators as members, each a
 * constant number of pointer writes.
 */
public class Deque {
    private var front: DPair? = null
    private var rear: DPair? = null

    public fun emptyDeque(): Boolean = front == null

    public fun frontDeque(): Value? = front?.item

    public fun rearDeque(): Value? = rear?.item

    public fun frontInsert(item: Value) {
        val cell = DPair(item, prev = null, next = front)
        val head = front
        if (head == null) {
            rear = cell
        } else {
            head.prev = cell
        }
        front = cell
    }

    public fun rearInsert(item: Value) {
        val cell = DPair(item, prev = rear, next = null)
        val tail = rear
        if (tail == null) {
            front = cell
        } else {
            tail.next = cell
        }
        rear = cell
    }

    public fun frontDelete(): Either<DequeError, Value> {
        val head = front ?: return Either.Left(DequeError.EmptyDeque)
        front = head.next
        if (front == null) {
            rear = null
        } else {
            front?.prev = null
        }
        return Either.Right(head.item)
    }

    public fun rearDelete(): Either<DequeError, Value> {
        val tail = rear ?: return Either.Left(DequeError.EmptyDeque)
        rear = tail.prev
        if (rear == null) {
            front = null
        } else {
            rear?.next = null
        }
        return Either.Right(tail.item)
    }
}

public fun makeDeque(): Deque = Deque()
