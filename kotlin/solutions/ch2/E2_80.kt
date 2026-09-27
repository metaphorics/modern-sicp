// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.80

package sicp.ch2.exercises

import arrow.core.raise.Raise

/**
 * Exercise 2.80: define a generic predicate `=zero?` and install it in
 * the generic arithmetic package, working for ordinary numbers, rational
 * numbers, and complex numbers. [installIsZero] installs it per level on
 * any table; [ex_2_80] proves each level's install on a fresh one. The
 * polynomial install that lets `adjoin-term` drop zero terms of nested
 * polys is exercise 2.87's own extension.
 *
 * Installs `=zero?` for the number levels of the tower.
 */
public fun installIsZero(table: NumTable) {
    table.put("=zero?", listOf("integer")) { args ->
        val z = args.single()
        if (z !is ZLong) raise(GenError.BadArgs("=zero?", "expected an integer"))
        ZLong(if (z.n == 0L) 1 else 0)
    }
    table.put("=zero?", listOf("rational")) { args ->
        val z = args.single()
        if (z !is QRat) raise(GenError.BadArgs("=zero?", "expected a rational"))
        ZLong(if (z.num.signum() == 0) 1 else 0)
    }
    table.put("=zero?", listOf("real")) { args ->
        val z = args.single()
        if (z !is Real) raise(GenError.BadArgs("=zero?", "expected a real"))
        ZLong(if (z.d == 0.0) 1 else 0)
    }
    table.put("=zero?", listOf("complex")) { args ->
        val z = args.single()
        if (z !is Complex) raise(GenError.BadArgs("=zero?", "expected a complex number"))
        ZLong(if (repRealPart(z.rep) == 0.0 && repImagPart(z.rep) == 0.0) 1 else 0)
    }
}

context(r: Raise<GenError>)
internal fun isZeroQ(
    table: NumTable,
    z: Num,
): Boolean = applyGeneric(table, "=zero?", listOf(z)) == ZLong(1)

/** Proves the predicate at every level: zero in each representation
 * answers true, its nonzero neighbor false. */
public fun ex_2_80(): List<Boolean> {
    val table = NumTable()
    installIsZero(table)
    return arrow.core.raise
        .either {
            listOf(
                isZeroQ(table, ZLong(0)),
                !isZeroQ(table, ZLong(2)),
                isZeroQ(table, qr(0, 5)),
                !isZeroQ(table, qr(1, 2)),
                isZeroQ(table, Real(0.0)),
                !isZeroQ(table, Real(-1.5)),
                isZeroQ(table, Complex(Rect(0.0, 0.0))),
                !isZeroQ(table, Complex(Rect(0.0, 3.0))),
            )
        }.getOrNull() ?: listOf()
}
