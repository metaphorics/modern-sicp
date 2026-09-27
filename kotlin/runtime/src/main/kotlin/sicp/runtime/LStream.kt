// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

/**
 * The stream of 3.5: a memoized head node whose tail is the book's
 * `cons-stream` delay pair. `LStream` is the host's own type rather than
 * `Sequence` because the book's streams are memoized and shareable — two
 * cursors over one stream force each tail once — and because the
 * self-referential streams of 3.5.4 (`ones`, the `solve` integral loop)
 * need a node a tail thunk can capture before it exists.
 */
public sealed interface LStream<out T> {
    /** The empty stream. */
    public data object Empty : LStream<Nothing>

    /** One stream node: a computed head and a memoized tail. */
    public class Cons<T>(
        /** The head value, computed eagerly. */
        public val head: T,
        tailThunk: () -> LStream<T>,
    ) : LStream<T> {
        /** The delayed tail, memoized on first force: the book's
         * `stream-cdr` runs the generator at most once. */
        public val tail: LStream<T> by lazy(LazyThreadSafetyMode.NONE, tailThunk)
    }
}

/** The book's `cons-stream head tail-thunk`. */
public fun <T> consStream(
    head: T,
    tail: () -> LStream<T>,
): LStream<T> = LStream.Cons(head, tail)

/** The book's `stream-car`; null on the empty stream. */
public fun <T> LStream<T>.streamHead(): T? = (this as? LStream.Cons)?.head

/** The book's `stream-cdr`; [LStream.Empty] stays empty. */
public fun <T> LStream<T>.streamTail(): LStream<T> =
    when (this) {
        is LStream.Empty -> LStream.Empty
        is LStream.Cons -> tail
    }

/** The first `n` heads, forcing memoized tails as it goes; infinite
 * streams are fine, callers `take` first. */
public fun <T> LStream<T>.take(n: Int): List<T> {
    val out = ArrayList<T>(n)
    var cursor = this
    while (out.size < n && cursor is LStream.Cons) {
        out.add(cursor.head)
        cursor = cursor.tail
    }
    return out
}

/** A cold view over the stream: each `asSequence` walks the same memoized
 * nodes, so forcing still happens once per node. */
public fun <T> LStream<T>.asSequence(): Sequence<T> =
    sequence {
        var cursor = this@asSequence
        while (cursor is LStream.Cons) {
            yield(cursor.head)
            cursor = cursor.tail
        }
    }
