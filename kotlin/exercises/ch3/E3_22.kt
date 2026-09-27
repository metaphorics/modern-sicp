// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.22

package sicp.ch3.exercises

import arrow.core.Either
import sicp.runtime.PendingSolution
import sicp.runtime.Value

/**
 * Exercise 3.22: a queue as a message-passing object with no state of
 * its own. The two pointers live in local `var`s of `makeQueue`, and
 * the returned object expression's members capture them -- the shape
 * of 3.1.1 applied to the queue of 3.3.2: state in the closure, not in
 * class fields.
 */
public interface MessagePassingQueue {
    public fun emptyQueue(): Boolean

    public fun insert(item: Value)

    public fun delete(): Either<QueueError, Value>

    public fun printQueue(): String
}

public fun makeQueue(): MessagePassingQueue = throw PendingSolution()
