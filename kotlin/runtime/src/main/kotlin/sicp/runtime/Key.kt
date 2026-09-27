// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.raise.Raise

/**
 * The hashable projection of [Value] behind the dynamic tables: the keys
 * the book's `put` and `get` compare with `equal?`. [Value] holds `Double`s
 * and closures, so the tables key on `Key` instead — the shapes `equal?`
 * compares in 2.4's dispatch table, 3.3.3's pairs-based table, and
 * `memo-fib`'s memoization.
 */
public sealed interface Key {
    /** A symbol key: `rectangular` in the table of 2.4.3. */
    public data class Sym(
        val name: String,
    ) : Key {
        public override fun toString(): String = name
    }

    /** An exact-integer key: `memo-fib`'s argument in 3.3.3. */
    public data class Int(
        val n: Long,
    ) : Key {
        public override fun toString(): String = n.toString()
    }

    /** A string key. */
    public data class Str(
        val s: String,
    ) : Key {
        public override fun toString(): String = "\"$s\""
    }

    /** A pair key: a list the pairs-based table compares elementwise. */
    public data class Pair(
        val car: Key,
        val cdr: Key,
    ) : Key {
        public override fun toString(): String = "($car . $cdr)"
    }

    /** The empty-list key: the terminator of a proper-list key. */
    public data object Nil : Key {
        public override fun toString(): String = "()"
    }
}

/** Projects a [Value] onto its hashable key shape. */
context(r: Raise<SchemeError>)
public fun keyOf(v: Value): Key =
    when (v) {
        is VSym -> Key.Sym(v.name)
        is VInt -> Key.Int(v.n)
        is VStr -> Key.Str(v.s)
        is VNil -> Key.Nil
        is VPair -> Key.Pair(keyOf(v.car), keyOf(v.cdr))
        else -> r.raise(SchemeError.TypeMismatch("not a table key: $v"))
    }
