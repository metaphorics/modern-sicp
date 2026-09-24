// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.93

package sicp.ch2.exercises

import arrow.core.raise.Raise
import kotlinx.collections.immutable.persistentListOf

/**
 * Exercise 2.93: modify the rational-arithmetic package to use generic
 * operations, but change `makeRat` so it does not reduce fractions to
 * lowest terms. Then `makeRational` of two polynomials is a rational
 * function, and adding one to itself leaves the common factor sitting in
 * both numerator and denominator -- the observation the exercise asks
 * for. The edition keeps the tower's exact rational [QRat] as its own
 * level, so this package installs under the tag `rational-function`.
 *
 * Assembles a rational function without reducing.
 */
public fun makeRationalFunction(
    num: Num,
    den: Num,
): Rat = Rat(num, den)

/** Installs the generic-coefficient rational-function package: every
 * operation dispatches to the tower's generic ones, and every fraction
 * the package assembles goes through `makeRat` -- the seam exercise
 * 2.97 later rewires to reduce. This installer's `makeRat` does not
 * reduce. */
public fun installRationalFunctionPackage(table: NumTable) {
    installRationalFunctionPackageBase(table) { n, d -> Rat(n, d) }
}

/** The package with the constructor seam exposed. */
public fun installRationalFunctionPackageBase(
    table: NumTable,
    makeRat: (Num, Num) -> Rat,
) {
    fun Raise<GenError>.twoRats(args: List<Num>): Pair<Rat, Rat> {
        val (a, b) = twoNums("rational-function arithmetic", args)
        if (a !is Rat || b !is Rat) raise(GenError.BadArgs("rational-function arithmetic", "expected two rational functions"))
        return a to b
    }
    table.put("add", listOf("rational-function", "rational-function")) { args ->
        val (a, b) = twoRats(args)
        makeRat(add(table, mul(table, a.num, b.den), mul(table, b.num, a.den)), mul(table, a.den, b.den))
    }
    table.put("sub", listOf("rational-function", "rational-function")) { args ->
        val (a, b) = twoRats(args)
        makeRat(sub(table, mul(table, a.num, b.den), mul(table, b.num, a.den)), mul(table, a.den, b.den))
    }
    table.put("mul", listOf("rational-function", "rational-function")) { args ->
        val (a, b) = twoRats(args)
        makeRat(mul(table, a.num, b.num), mul(table, a.den, b.den))
    }
    table.put("div", listOf("rational-function", "rational-function")) { args ->
        val (a, b) = twoRats(args)
        makeRat(mul(table, a.num, b.den), mul(table, a.den, b.num))
    }
    table.put("make", listOf("rational-function")) { args ->
        val (a, b) = twoNums("make", args)
        makeRat(a, b)
    }
}

context(r: Raise<GenError>)
internal fun addRat(
    table: NumTable,
    a: Num,
    b: Num,
): Num = applyGeneric(table, "add", listOf(a, b))

/** Builds the book's rational function `(x^3 + 1)/(x^2 + 1)`, adds it to
 * itself with the generic `add`, and reports the unreduced result: the
 * factor `x^2 + 1` still sits in both parts. */
public fun ex_2_93(): String {
    val table = NumTable()
    installGenericArithmetic(table)
    installNeg(table)
    installPolynomialPackage(table)
    installPolyIsZero(table)
    installRationalFunctionPackage(table)
    val p1 = makePolynomial("x", listOf(Term(2, ZLong(1)), Term(0, ZLong(1))))
    val p2 = makePolynomial("x", listOf(Term(3, ZLong(1)), Term(0, ZLong(1))))
    return arrow.core.raise
        .either {
            val rf = makeRationalFunction(p2, p1)
            val doubled = addRat(table, rf, rf)
            show(doubled)
        }.getOrNull() ?: "unreachable"
}
