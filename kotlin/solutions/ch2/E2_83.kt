// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.83

package sicp.ch2.exercises

import arrow.core.raise.Raise

/**
 * Exercise 2.83: for the tower of Figure 2.25 -- integer, rational, real,
 * complex -- design a `raise` operation for each level except the top,
 * and install a generic `raise` that works for each of them.
 *
 * Installs `raise` for every level below the top: integers promote to
 * exact rationals, rationals to reals (inexact, by division), reals to
 * complexes with zero imaginary part, and the overflow level `BigZ` to
 * rationals too.
 */
public fun installRaise(table: NumTable) {
    table.put("raise", listOf("integer")) { args ->
        val z = args.single()
        if (z !is ZLong) raise(GenError.BadArgs("raise", "expected an integer"))
        QRat(z.n.toBigInteger(), java.math.BigInteger.ONE)
    }
    table.put("raise", listOf("bigint")) { args ->
        val z = args.single()
        if (z !is BigZ) raise(GenError.BadArgs("raise", "expected a bigint"))
        QRat(z.n, java.math.BigInteger.ONE)
    }
    table.put("raise", listOf("rational")) { args ->
        val z = args.single()
        if (z !is QRat) raise(GenError.BadArgs("raise", "expected a rational"))
        Real(z.num.toDouble() / z.den.toDouble())
    }
    table.put("raise", listOf("real")) { args ->
        val z = args.single()
        if (z !is Real) raise(GenError.BadArgs("raise", "expected a real"))
        Complex(Rect(z.d, 0.0))
    }
}

/** The generic `raise` operation: dispatches on the value's own level. */
context(r: Raise<GenError>)
public fun raiseOf(
    table: NumTable,
    z: Num,
): Num = applyGeneric(table, "raise", listOf(z))

/** Raises one value of each level and reports the levels reached, the
 * book's check that each install answers. */
public fun ex_2_83(): List<String> {
    val table = NumTable()
    installRaise(table)
    return arrow.core.raise
        .either {
            listOf(
                show(raiseOf(table, ZLong(7))),
                show(raiseOf(table, qr(3, 4))),
                show(raiseOf(table, Real(2.5))),
            )
        }.getOrNull() ?: listOf()
}
