// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.79

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlin.reflect.KClass

/**
 * Install numeric equality for every tower level. [equvAcross] additionally
 * compares values from different levels by raising them to a common level.
 */
public fun installEquQueries(table: NumTable) {
    table.put("equ?", listOf("integer", "integer")) { args ->
        val (a, b) = twoNums("equ?", args)
        if (a is ZLong && b is ZLong) ZLong(if (a.n == b.n) 1 else 0) else raise(GenError.BadArgs("equ?", "expected two integers"))
    }
    table.put("equ?", listOf("rational", "rational")) { args ->
        val (a, b) = twoNums("equ?", args)
        if (a is QRat && b is QRat) {
            ZLong(if (a.num.multiply(b.den) == b.num.multiply(a.den)) 1 else 0)
        } else {
            raise(GenError.BadArgs("equ?", "expected two rationals"))
        }
    }
    table.put("equ?", listOf("real", "real")) { args ->
        val (a, b) = twoNums("equ?", args)
        if (a is Real && b is Real) ZLong(if (a.d == b.d) 1 else 0) else raise(GenError.BadArgs("equ?", "expected two reals"))
    }
    table.put("equ?", listOf("complex", "complex")) { args ->
        val (a, b) = twoNums("equ?", args)
        if (a is Complex && b is Complex) {
            val equal = repRealPart(a.rep) == repRealPart(b.rep) && repImagPart(a.rep) == repImagPart(b.rep)
            ZLong(if (equal) 1 else 0)
        } else {
            raise(GenError.BadArgs("equ?", "expected two complex numbers"))
        }
    }
}

/** The raise chain the tower levels form: each level's way up. */
public val towerRaiseChain: Map<KClass<*>, Pair<KClass<*>, (Num) -> Num>> =
    mapOf(
        ZLong::class to Pair(QRat::class) { z: Num -> QRat((z as ZLong).n.toBigInteger(), java.math.BigInteger.ONE) },
        QRat::class to
            Pair(Real::class) { z: Num ->
                val q = z as QRat
                Real(q.num.toDouble() / q.den.toDouble())
            },
        Real::class to Pair(Complex::class) { z: Num -> Complex(Rect((z as Real).d, 0.0)) },
    )

/** Raises `z` one level, or null at the top of the tower. */
public fun raiseOnce(z: Num): Num? {
    val step = towerRaiseChain[z::class] ?: return null
    return step.second(z)
}

context(r: Raise<GenError>)
private fun equAtSameLevel(
    table: NumTable,
    x: Num,
    y: Num,
): Boolean = applyGeneric(table, "equ?", listOf(x, y)) == ZLong(1)

/** Compare values at different tower levels after raising them to a common level. */
context(r: Raise<GenError>)
public fun equvAcross(
    table: NumTable,
    x: Num,
    y: Num,
): Boolean {
    var a = x
    var b = y
    var steps = 0
    while (typeTagOf(a) != typeTagOf(b)) {
        if (steps++ > 8) r.raise(GenError.BadArgs("equ?", "levels never meet"))
        a = raiseOnce(a) ?: a
        b = raiseOnce(b) ?: b
    }
    return equAtSameLevel(table, a, b)
}

/** Run same-level equality checks and the cross-level extension. */
public fun ex_2_79(): Boolean {
    val table = NumTable()
    installEquQueries(table)
    val same =
        arrow.core.raise.either {
            listOf(
                equvAcross(table, ZLong(3), ZLong(3)),
                !equvAcross(table, ZLong(3), ZLong(4)),
                equvAcross(table, qr(1, 2), qr(2, 4)),
                equvAcross(table, Complex(Rect(5.0, 0.0)), Complex(Polar(5.0, 0.0))) && !equvAcross(table, ZLong(1), qr(1, 2)),
            )
        }
    val cross =
        arrow.core.raise.either {
            listOf(
                equvAcross(table, ZLong(3), qr(6, 2)),
                equvAcross(table, ZLong(3), Real(3.0)),
                equvAcross(table, qr(3, 1), Complex(Rect(3.0, 0.0))),
            )
        }
    return (same.getOrNull() ?: listOf(false)).all { it } && (cross.getOrNull() ?: listOf(false)).all { it }
}
