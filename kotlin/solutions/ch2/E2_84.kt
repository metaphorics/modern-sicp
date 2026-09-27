// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.84

package sicp.ch2.exercises

import arrow.core.raise.Raise

/**
 * Exercise 2.84: using the `raise` operation of exercise 2.83, revise
 * `apply-generic` so it coerces its arguments to a common type by
 * successive raising. Which of two types is higher is tested the
 * compatible way -- by trying to raise one value toward the other's level
 * -- so adding a new level to the tower needs no new central table, only
 * that level's own `raise` install.
 *
 * The highest number of raises a value may need to reach any other
 * level of the tower.
 */
public const val TOWER_HEIGHT: Int = 4

/** Raises `z` toward `targetTag` until it lands there; null when the
 * tower tops out first. */
public fun raiseToward(
    table: NumTable,
    z: Num,
    targetTag: String,
): Num? {
    var current: Num = z
    repeat(TOWER_HEIGHT) {
        if (typeTagOf(current) == targetTag) return current
        current = arrow.core.raise
            .either { raiseOf(table, current) }
            .getOrNull() ?: return null
    }
    return if (typeTagOf(current) == targetTag) current else null
}

/** Lifts the lower of the two arguments to the higher's level, as a
 * two-element argument list; null when no level meets. */
public fun raiseToMeet(
    table: NumTable,
    a: Num,
    b: Num,
): List<Num>? {
    if (typeTagOf(a) == typeTagOf(b)) return listOf(a, b)
    raiseToward(table, a, typeTagOf(b))?.let { return listOf(it, b) }
    raiseToward(table, b, typeTagOf(a))?.let { return listOf(a, it) }
    return null
}

/** The 2.84 revision of `apply-generic`: on a two-argument miss, raise
 * the lower argument until the levels agree, then retry. */
context(r: Raise<GenError>)
public fun applyGenericRaising(
    table: NumTable,
    op: String,
    args: List<Num>,
): Num {
    table.get(op, args.map { typeTagOf(it) })?.let { return it.invoke(r, args) }
    if (args.size == 2 && typeTagOf(args[0]) != typeTagOf(args[1])) {
        val met = raiseToMeet(table, args[0], args[1])
        if (met != null) return applyGenericRaising(table, op, met)
    }
    return r.raise(GenError.NoMethod(op, args.map { typeTagOf(it) }))
}

context(r: Raise<GenError>)
internal fun addRaising(
    table: NumTable,
    x: Num,
    y: Num,
): Num = applyGenericRaising(table, "add", listOf(x, y))

context(r: Raise<GenError>)
internal fun mulRaising(
    table: NumTable,
    x: Num,
    y: Num,
): Num = applyGenericRaising(table, "mul", listOf(x, y))

/** Works mixed-level operations by successive raising: integer plus
 * rational lands in the rational package, integer plus real in the real
 * package, integer plus complex in the complex package. */
public fun ex_2_84(): Boolean {
    val table = NumTable()
    installGenericArithmetic(table)
    installRaise(table)
    installRealPackage(table)
    return arrow.core.raise
        .either {
            val a = addRaising(table, ZLong(1), qr(1, 2))
            val b = addRaising(table, ZLong(1), Real(2.5))
            val c = addRaising(table, ZLong(1), Complex(Rect(0.0, 1.0)))
            a == qr(3, 2) && b == Real(3.5) && c == Complex(Rect(1.0, 1.0))
        }.getOrNull() ?: false
}
