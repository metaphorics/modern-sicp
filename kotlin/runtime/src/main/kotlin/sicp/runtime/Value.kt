// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList

/** Legacy datum API retained until its chapter callers migrate to [Datum]. */
public sealed interface Value

@JvmInline
public value class VInt(
    public val n: Long,
) : Value {
    public override fun toString(): String = n.toString()
}

@JvmInline
public value class VReal(
    public val d: Double,
) : Value {
    public override fun toString(): String = d.toString()
}

@JvmInline
public value class VBool(
    public val b: Boolean,
) : Value

@JvmInline
public value class VSym(
    public val name: String,
) : Value

@JvmInline
public value class VStr(
    public val s: String,
) : Value

public data object VNil : Value

public class VPair(
    car: Value,
    cdr: Value,
) : Value {
    public var car: Value = car
        internal set

    public var cdr: Value = cdr
        internal set
}

public fun cons(
    car: Value,
    cdr: Value,
): VPair = VPair(car, cdr)

public fun VPair.setCar(v: Value) {
    car = v
}

public fun VPair.setCdr(v: Value) {
    cdr = v
}

context(r: Raise<DataError>)
public fun car(v: Value): Value =
    when (v) {
        is VPair -> v.car
        else -> r.raise(DataError.TypeMismatch("car of a non-pair: $v"))
    }

context(r: Raise<DataError>)
public fun cdr(v: Value): Value =
    when (v) {
        is VPair -> v.cdr
        else -> r.raise(DataError.TypeMismatch("cdr of a non-pair: $v"))
    }

public data class VTagged(
    val tag: String,
    val data: Value,
) : Value

public fun equalv(
    a: Value,
    b: Value,
): Boolean =
    when {
        a is VPair && b is VPair -> equalv(a.car, b.car) && equalv(a.cdr, b.cdr)
        else -> a == b
    }

public fun vlist(items: List<Value>): Value = items.foldRight(VNil as Value) { value, tail -> cons(value, tail) }

public fun vlist(vararg items: Value): Value = vlist(items.toList())

context(r: Raise<DataError>)
public fun listItems(v: Value): PersistentList<Value> {
    val items = mutableListOf<Value>()
    var cursor = v
    while (cursor is VPair) {
        items.add(cursor.car)
        cursor = cursor.cdr
    }
    if (cursor is VNil) return items.toPersistentList()
    r.raise(DataError.TypeMismatch("not a proper list: $v"))
}

public fun isTrue(v: Value): Boolean = v != VBool(false)

public val noParams: PersistentList<String> = persistentListOf()
