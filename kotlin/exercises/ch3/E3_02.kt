// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.2

package sicp.ch3.exercises

import sicp.runtime.PendingSolution

/**
 * The book's dispatch protocol for a monitored function: instead of
 * funneling `how-many-calls?` and `reset-count` through the one argument
 * `f` itself takes, this edition gives each of the book's three special
 * requests its own named method.
 */
public interface Monitored<T, R> {
    public fun call(x: T): R

    public fun howManyCalls(): Int

    public fun resetCount()
}

/**
 * Exercise 3.2: In software-testing applications, it is useful to be able
 * to count the number of times a given function is called during the
 * course of a computation. Write a function `makeMonitored` that takes as
 * input a function `f` that itself takes one input. The result returned
 * by `makeMonitored` keeps track of the number of times it has been
 * called by maintaining an internal counter, answers that counter through
 * `howManyCalls()`, resets it to zero through `resetCount()`, and for any
 * other call through `call(x)` returns the result of calling `f` on `x`
 * and increments the counter. For instance, a monitored version of
 * `sqrt`, `s`, answers `s.call(100.0)` with 10.0 and `s.howManyCalls()`
 * with 1.
 *
 * The scaffold's `call` always forwards to `f` without counting anything.
 */
public fun <T, R> makeMonitored(f: (T) -> R): Monitored<T, R> = throw PendingSolution()
