// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.23

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.PendingSolution
import sicp.runtime.Value

public sealed interface DequeError {
    public data object EmptyDeque : DequeError
}

/**
 * Exercise 3.23: a deque ("double-ended queue") over doubly linked
 * cells. One cell carries its item, the previous cell, and the next
 * cell, so both ends are one pointer away and every operation is
 * constant time. The class keeps the two end pointers; the operations
 * are its members, named after the book's make-deque, empty-deque?,
 * front-deque, rear-deque, front-insert-deque!, rear-insert-deque!,
 * front-delete-deque!, and rear-delete-deque!.
 */
public class DPair(
    public var item: Value,
    public var prev: DPair?,
    public var next: DPair?,
)

public class Deque {
    public fun emptyDeque(): Boolean = throw PendingSolution()

    public fun frontDeque(): Value? = throw PendingSolution()

    public fun rearDeque(): Value? = throw PendingSolution()

    public fun frontInsert(item: Value): Unit = throw PendingSolution()

    public fun rearInsert(item: Value): Unit = throw PendingSolution()

    public fun frontDelete(): Either<DequeError, Value> = throw PendingSolution()

    public fun rearDelete(): Either<DequeError, Value> = throw PendingSolution()
}

public fun makeDeque(): Deque = throw PendingSolution()
