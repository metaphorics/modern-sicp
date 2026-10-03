// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.23

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.Datum
import sicp.runtime.PendingSolution

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
    public var item: Datum,
    public var prev: DPair?,
    public var next: DPair?,
)

public class Deque {
    public fun emptyDeque(): Boolean = throw PendingSolution()

    public fun frontDeque(): Datum? = throw PendingSolution()

    public fun rearDeque(): Datum? = throw PendingSolution()

    public fun frontInsert(item: Datum): Unit = throw PendingSolution()

    public fun rearInsert(item: Datum): Unit = throw PendingSolution()

    public fun frontDelete(): Either<DequeError, Datum> = throw PendingSolution()

    public fun rearDelete(): Either<DequeError, Datum> = throw PendingSolution()
}

public fun makeDeque(): Deque = throw PendingSolution()
