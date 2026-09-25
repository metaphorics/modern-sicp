// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.70: the let bindings snapshot the old collection
// before the new one is installed; without them the tail reads the
// mutated cell and the stream grows into a cycle.

package sicp.ch4.solutions

import sicp.ch4.listStream
import sicp.runtime.LStream
import sicp.runtime.consStream
import sicp.runtime.take

/** The book's stream-of-ones shape: a memoized tail that reads the very
 * stream being defined. */
private fun ones(): LStream<Long> = consStream(1L) { ones() }

public fun onesModel(): List<Long> = ones().take(4)

/** The broken add: the tail thunk reads the box AFTER the assignment,
 * so the stream contains itself; the stored (a b) is unreachable. */
public fun brokenAddDemo(): List<String> {
    var stream: LStream<String> = listStream(listOf("a", "b"))
    // (set! THE-ASSERTIONS (cons-stream assertion THE-ASSERTIONS))
    stream = consStream("c") { stream }
    return stream.take(4)
}

/** The edition's bind-then-append discipline: the right side is
 * evaluated before the cell is updated, so the new stream is c followed
 * by exactly the old a b. */
public fun letBoundAddDemo(): List<String> {
    var stream: LStream<String> = listStream(listOf("a", "b"))
    // (let ((old THE-ASSERTIONS)) (set! THE-ASSERTIONS (cons-stream c old)))
    val old = stream
    stream = consStream("c") { old }
    return stream.take(4)
}

public fun letPurposeDemo(): List<String> {
    val out = mutableListOf<String>()
    out.add("ones model: take(4) = ${onesModel()}")
    out.add("broken add-assertion! on (a b): add c => take(4) = ${brokenAddDemo()}")
    out.add("let-bound add-assertion! on (a b): add c => take(4) = ${letBoundAddDemo()}")
    out.add("the book's hazard: the memoized tail reads THE-ASSERTIONS at force time, after set! has rebound the name")
    out.add("the edition's addAssertion binds the old collection before appending; the hazard cannot arise")
    return out
}
