// SPDX-License-Identifier: GPL-3.0-only
package sicp.runtime

import arrow.core.raise.Raise
import java.util.ArrayDeque
import java.util.IdentityHashMap

/** Host-language data used by the translated lessons, never guest source. */
public sealed interface Datum

/** An exact whole number represented at the host's declared width. */
public data class Whole(
    public val value: Long,
) : Datum {
    public override fun toString(): String = renderDatum(this)
}

/** An inexact real number with native Kotlin/JVM `Double` equality. */
public data class Real(
    public val value: Double,
) : Datum {
    public override fun toString(): String = renderDatum(this)
}

/** A native Boolean value. */
public data class Truth(
    public val value: Boolean,
) : Datum {
    public override fun toString(): String = renderDatum(this)
}

/** A symbolic name represented as host data, not parsed source. */
public data class Symbol(
    public val name: String,
) : Datum {
    public override fun toString(): String = renderDatum(this)
}

/** Text data. */
public data class Text(
    public val value: String,
) : Datum {
    public override fun toString(): String = renderDatum(this)
}

/** Terminator for proper datum lists. */
public data object Empty : Datum {
    public override fun toString(): String = renderDatum(this)
}

/** A mutable pair cell. Pair identity and aliases are preserved. */
public class PairCell(
    public var first: Datum,
    public var second: Datum,
) : Datum {
    public override fun toString(): String = renderDatum(this)
}

/** A tagged datum used by data-directed operation examples. */
public data class Tagged(
    public val tag: String,
    public val payload: Datum,
) : Datum {
    public override fun toString(): String = renderDatum(this)
}

/**
 * Canonical, non-Scheme representation used by [Datum.toString]. Composite
 * values use constructor-shaped text; repeated pair cells on the active path
 * render as `<cycle>`. Strings escape quotes, backslashes, and control chars.
 */
public fun renderDatum(value: Datum): String {
    val output = StringBuilder()
    val tasks = ArrayDeque<RenderTask>()
    val active = IdentityHashMap<PairCell, Boolean>()
    tasks.addLast(RenderTask.Visit(value))
    while (tasks.isNotEmpty()) {
        when (val task = tasks.removeLast()) {
            is RenderTask.Append -> {
                output.append(task.text)
            }

            is RenderTask.FinishPair -> {
                active.remove(task.pair)
            }

            is RenderTask.Visit -> {
                when (val datum = task.datum) {
                    is Whole -> {
                        output.append("Whole(value=").append(datum.value).append(')')
                    }

                    is Real -> {
                        output.append("Real(value=").append(datum.value).append(')')
                    }

                    is Truth -> {
                        output.append("Truth(value=").append(datum.value).append(')')
                    }

                    is Symbol -> {
                        output.append("Symbol(name=")
                        appendQuoted(datum.name, output)
                        output.append(')')
                    }

                    is Text -> {
                        output.append("Text(value=")
                        appendQuoted(datum.value, output)
                        output.append(')')
                    }

                    Empty -> {
                        output.append("Empty")
                    }

                    is PairCell -> {
                        if (active.put(datum, true) != null) {
                            output.append("<cycle>")
                            continue
                        }
                        tasks.addLast(RenderTask.FinishPair(datum))
                        tasks.addLast(RenderTask.Append(")"))
                        tasks.addLast(RenderTask.Visit(datum.second))
                        tasks.addLast(RenderTask.Append(", second="))
                        tasks.addLast(RenderTask.Visit(datum.first))
                        tasks.addLast(RenderTask.Append("PairCell(first="))
                    }

                    is Tagged -> {
                        output.append("Tagged(tag=")
                        appendQuoted(datum.tag, output)
                        output.append(", payload=")
                        tasks.addLast(RenderTask.Append(")"))
                        tasks.addLast(RenderTask.Visit(datum.payload))
                    }
                }
            }
        }
    }
    return output.toString()
}

private sealed interface RenderTask {
    public data class Visit(
        public val datum: Datum,
    ) : RenderTask

    public data class Append(
        public val text: String,
    ) : RenderTask

    public data class FinishPair(
        public val pair: PairCell,
    ) : RenderTask
}

private fun appendQuoted(
    value: String,
    output: StringBuilder,
) {
    output.append('"')
    for (character in value) {
        when (character) {
            '"' -> {
                output.append("\\\"")
            }

            '\\' -> {
                output.append("\\\\")
            }

            '\n' -> {
                output.append("\\n")
            }

            '\r' -> {
                output.append("\\r")
            }

            '\t' -> {
                output.append("\\t")
            }

            else -> {
                if (Character.isISOControl(character)) {
                    output.append("\\u").append(character.code.toString(16).padStart(4, '0'))
                } else {
                    output.append(character)
                }
            }
        }
    }
    output.append('"')
}

/** Construct one mutable pair without converting or copying either element. */
public fun pair(
    first: Datum,
    second: Datum,
): PairCell = PairCell(first, second)

/** Construct a proper list while retaining every item reference. */
public fun datumList(vararg items: Datum): Datum = items.foldRight(Empty as Datum) { item, tail -> PairCell(item, tail) }

/**
 * Return the elements of a finite proper list in order. Improper and cyclic
 * pair chains raise [DatumError.TypeMismatch]. Pair objects in the elements
 * remain the original objects.
 */
context(r: Raise<DatumError>)
public fun elements(value: Datum): List<Datum> {
    val result = ArrayList<Datum>()
    val seen = IdentityHashMap<PairCell, Unit>()
    var cursor = value
    while (cursor is PairCell) {
        if (seen.put(cursor, Unit) != null) {
            r.raise(DatumError.TypeMismatch("not a finite proper list", listOf(value)))
        }
        result.add(cursor.first)
        cursor = cursor.second
    }
    if (cursor === Empty) return result
    r.raise(DatumError.TypeMismatch("not a proper list", listOf(value)))
}

/** Compare datum structure without copying pair cells or recursing on the JVM stack. */
public fun structurallyEqual(
    first: Datum,
    second: Datum,
): Boolean {
    if (first === second) return true
    if (first !is PairCell && first !is Tagged) return first == second

    val pending = ArrayDeque<Datum>()
    val compared = IdentityHashMap<PairCell, IdentityHashMap<PairCell, Boolean>>()
    pending.addLast(first)
    pending.addLast(second)
    while (pending.isNotEmpty()) {
        val right = pending.removeLast()
        val left = pending.removeLast()
        if (left === right) continue
        when {
            left is PairCell && right is PairCell -> {
                val rightPairs = compared.getOrPut(left) { IdentityHashMap() }
                if (rightPairs.put(right, true) == null) {
                    pending.addLast(left.first)
                    pending.addLast(right.first)
                    pending.addLast(left.second)
                    pending.addLast(right.second)
                }
            }

            left is Tagged && right is Tagged -> {
                if (left.tag != right.tag) return false
                pending.addLast(left.payload)
                pending.addLast(right.payload)
            }

            else -> {
                if (left != right) return false
            }
        }
    }
    return true
}
