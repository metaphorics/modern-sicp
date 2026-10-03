// SPDX-License-Identifier: GPL-3.0-only
package sicp.runtime

import arrow.core.raise.Raise
import java.util.ArrayDeque
import java.util.IdentityHashMap

private const val SYMBOL_KEY_HASH_TAG = 1
private const val INTEGER_KEY_HASH_TAG = 2
private const val TEXT_KEY_HASH_TAG = 3
private const val PAIR_KEY_HASH_TAG = 4
private const val EMPTY_KEY_HASH_TAG = 5

/** Hashable data shapes admitted as keys by native data-directed examples. */
public sealed interface DatumKey {
    public data class Symbol(
        public val name: String,
    ) : DatumKey

    public data class Integer(
        public val value: Long,
    ) : DatumKey

    public data class Text(
        public val value: String,
    ) : DatumKey

    public data class Pair(
        public val first: DatumKey,
        public val second: DatumKey,
    ) : DatumKey {
        public override fun equals(other: Any?): Boolean = other is Pair && datumKeyEquals(this, other)

        public override fun hashCode(): Int = datumKeyHashCode(this)
    }

    public data object Empty : DatumKey
}

private fun datumKeyEquals(
    first: DatumKey,
    second: DatumKey,
): Boolean {
    if (first === second) return true
    val pending = ArrayDeque<DatumKey>()
    pending.addLast(first)
    pending.addLast(second)
    while (pending.isNotEmpty()) {
        val right = pending.removeLast()
        val left = pending.removeLast()
        if (left === right) continue
        when {
            left is DatumKey.Pair && right is DatumKey.Pair -> {
                pending.addLast(left.second)
                pending.addLast(right.second)
                pending.addLast(left.first)
                pending.addLast(right.first)
            }

            left is DatumKey.Pair || right is DatumKey.Pair -> {
                return false
            }

            else -> {
                if (left != right) return false
            }
        }
    }
    return true
}

private fun datumKeyHashCode(value: DatumKey): Int {
    var hash = 1
    val pending = ArrayDeque<DatumKey>()
    pending.addLast(value)
    while (pending.isNotEmpty()) {
        when (val key = pending.removeLast()) {
            is DatumKey.Symbol -> {
                hash = 31 * hash + SYMBOL_KEY_HASH_TAG
                hash = 31 * hash + key.name.hashCode()
            }

            is DatumKey.Integer -> {
                hash = 31 * hash + INTEGER_KEY_HASH_TAG
                hash = 31 * hash + key.value.hashCode()
            }

            is DatumKey.Text -> {
                hash = 31 * hash + TEXT_KEY_HASH_TAG
                hash = 31 * hash + key.value.hashCode()
            }

            is DatumKey.Pair -> {
                hash = 31 * hash + PAIR_KEY_HASH_TAG
                pending.addLast(key.second)
                pending.addLast(key.first)
            }

            DatumKey.Empty -> {
                hash = 31 * hash + EMPTY_KEY_HASH_TAG
            }
        }
    }
    return hash
}

/** Project a supported host datum onto a stable operation-table key. */
context(r: Raise<DatumError>)
public fun keyOf(value: Datum): DatumKey {
    val tasks = ArrayDeque<KeyTask>()
    val keys = ArrayDeque<DatumKey>()
    val active = IdentityHashMap<PairCell, Boolean>()
    tasks.addLast(KeyTask.Visit(value))
    while (tasks.isNotEmpty()) {
        when (val task = tasks.removeLast()) {
            is KeyTask.BuildPair -> {
                val second = keys.removeLast()
                val first = keys.removeLast()
                keys.addLast(DatumKey.Pair(first, second))
                active.remove(task.pair)
            }

            is KeyTask.Visit -> {
                when (val datum = task.datum) {
                    is Symbol -> {
                        keys.addLast(DatumKey.Symbol(datum.name))
                    }

                    is Whole -> {
                        keys.addLast(DatumKey.Integer(datum.value))
                    }

                    is Text -> {
                        keys.addLast(DatumKey.Text(datum.value))
                    }

                    Empty -> {
                        keys.addLast(DatumKey.Empty)
                    }

                    is PairCell -> {
                        if (active.put(datum, true) != null) {
                            r.raise(DatumError.TypeMismatch("cyclic datum cannot be a table key", listOf(datum)))
                        }
                        tasks.addLast(KeyTask.BuildPair(datum))
                        tasks.addLast(KeyTask.Visit(datum.second))
                        tasks.addLast(KeyTask.Visit(datum.first))
                    }

                    else -> {
                        r.raise(DatumError.TypeMismatch("not a table key: $datum", listOf(datum)))
                    }
                }
            }
        }
    }
    return keys.removeLast()
}

private sealed interface KeyTask {
    public data class Visit(
        public val datum: Datum,
    ) : KeyTask

    public data class BuildPair(
        public val pair: PairCell,
    ) : KeyTask
}
