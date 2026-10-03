// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.raise.Raise

/** Transitional key shapes for the legacy [Value] API. */
public sealed interface Key {
    public data class Sym(
        val name: String,
    ) : Key

    public data class Int(
        val n: Long,
    ) : Key

    public data class Str(
        val s: String,
    ) : Key

    public data class Pair(
        val car: Key,
        val cdr: Key,
    ) : Key

    public data object Nil : Key
}

/** Project a legacy value to its legacy key shape. */
context(r: Raise<DataError>)
public fun keyOf(v: Value): Key =
    when (v) {
        is VSym -> Key.Sym(v.name)
        is VInt -> Key.Int(v.n)
        is VStr -> Key.Str(v.s)
        is VNil -> Key.Nil
        is VPair -> Key.Pair(keyOf(v.car), keyOf(v.cdr))
        else -> r.raise(DataError.TypeMismatch("not a table key: $v"))
    }
