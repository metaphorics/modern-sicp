// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.85

package sicp.ch2.exercises

import arrow.core.raise.Raise

/**
 * Exercise 2.85: design `drop`, which lowers a tower value to its
 * simplest representation. The test for lowerable: `project` the value,
 * raise the projection back, and compare with the generic `equ?` -- if
 * the round trip reproduces the value, it can be dropped. `1.5 + 0i`
 * drops as far as `real`, `1 + 0i` as far as `integer`, `2 + 3i` not at
 * all. Finally `applyGenericDropping` rewrites the raising dispatch of
 * exercise 2.84 so every answer leaves the tower simplified.
 *
 * Installs `project`: complexes drop their imaginary part, reals round
 * to integers, integral rationals drop their unit denominator. A level
 * with no projection (integers) raises `NoMethod`, which [dropNum] reads
 * as "cannot be lowered".
 */
public fun installProject(table: NumTable) {
    table.put("project", listOf("complex")) { args ->
        val z = args.single()
        if (z !is Complex) raise(GenError.BadArgs("project", "expected a complex number"))
        Real(repRealPart(z.rep))
    }
    table.put("project", listOf("real")) { args ->
        val z = args.single()
        if (z !is Real) raise(GenError.BadArgs("project", "expected a real"))
        ZLong(Math.round(z.d))
    }
    table.put("project", listOf("rational")) { args ->
        val z = args.single()
        if (z !is QRat) raise(GenError.BadArgs("project", "expected a rational"))
        if (z.den != java.math.BigInteger.ONE) raise(GenError.BadArgs("project", "non-integral rational"))
        ZLong(z.num.longValueExact())
    }
}

context(r: Raise<GenError>)
internal fun projectOf(
    table: NumTable,
    z: Num,
): Num = applyGeneric(table, "project", listOf(z))

/** Drops `z` as far as it goes: project, raise back, compare; unequal or
 * unprojectable values stay put. */
context(r: Raise<GenError>)
public fun dropNum(
    table: NumTable,
    z: Num,
): Num {
    val lowered =
        arrow.core.raise
            .either { projectOf(table, z) }
            .getOrNull() ?: return z
    val back =
        arrow.core.raise
            .either<GenError, Num> { raiseToward(table, lowered, typeTagOf(z)) ?: z }
            .getOrNull() ?: return z
    val same =
        arrow.core.raise
            .either { equv(table, back, z) }
            .getOrNull() ?: false
    return if (same) dropNum(table, lowered) else z
}

/** The 2.85 revision of the dispatch: raise to a common level, operate,
 * then simplify the answer on the way out. */
context(r: Raise<GenError>)
public fun applyGenericDropping(
    table: NumTable,
    op: String,
    args: List<Num>,
): Num = dropNum(table, applyGenericRaising(table, op, args))

context(r: Raise<GenError>)
internal fun addDropping(
    table: NumTable,
    x: Num,
    y: Num,
): Num = applyGenericDropping(table, "add", listOf(x, y))

/** Runs the book's three probes -- `1.5 + 0i` to real, `1 + 0i` to
 * integer, `2 + 3i` unchanged -- plus a mixed addition whose answer comes
 * back an integer. */
public fun ex_2_85(): List<String> {
    val table = NumTable()
    installGenericArithmetic(table)
    installRaise(table)
    installProject(table)
    return arrow.core.raise
        .either {
            listOf(
                show(dropNum(table, Complex(Rect(1.5, 0.0)))),
                show(dropNum(table, Complex(Rect(1.0, 0.0)))),
                show(dropNum(table, Complex(Rect(2.0, 3.0)))),
                show(addDropping(table, ZLong(2), Complex(Rect(4.0, 0.0)))),
            )
        }.getOrNull() ?: listOf()
}
