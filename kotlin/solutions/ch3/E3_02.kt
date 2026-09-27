// SPDX-License-Identifier: GPL-3.0-only
// Chapter 3, exercise 3.2

package sicp.ch3.exercises

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

/** A captured `var count`, incremented on every real call and left alone by the two bookkeeping requests. */
public fun <T, R> makeMonitored(f: (T) -> R): Monitored<T, R> {
    var count = 0
    return object : Monitored<T, R> {
        override fun call(x: T): R {
            count += 1
            return f(x)
        }

        override fun howManyCalls(): Int = count

        override fun resetCount() {
            count = 0
        }
    }
}
