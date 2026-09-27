// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.5.3: example, symbolic algebra

package sicp.ch2.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch2.exercises.NumTable
import sicp.ch2.exercises.Term
import sicp.ch2.exercises.ZLong
import sicp.ch2.exercises.addPoly
import sicp.ch2.exercises.installGenericArithmetic
import sicp.ch2.exercises.installPolynomialPackage
import sicp.ch2.exercises.makePolynomial
import sicp.ch2.exercises.mulPoly
import sicp.ch2.exercises.show

/**
 * 2.5.3's own example polynomial, `5x^2 + 3x + 7`, is a poly: a
 * variable and a term list. `add-poly` and `mul-poly` combine two polys
 * under the same variable by combining their term lists, then get
 * installed as the generic `add` and `mul` for type `polynomial` so
 * they answer through the same front door as every other level.
 * [makeFiveXSquaredPlusThreeXPlusSeven] is the book's own polynomial;
 * [makeXPlusOne] is a second, simple one to combine it with.
 */
private fun freshTable(): NumTable {
    val table = NumTable()
    installGenericArithmetic(table)
    installPolynomialPackage(table)
    return table
}

/** The book's own `5x^2 + 3x + 7`, from 2.5.3's introduction of what a
 * polynomial is. */
public fun makeFiveXSquaredPlusThreeXPlusSeven() = makePolynomial("x", listOf(Term(2, ZLong(5)), Term(1, ZLong(3)), Term(0, ZLong(7))))

/** A second, simple polynomial in the same variable to add and multiply
 * against the book's own example. */
public fun makeXPlusOne() = makePolynomial("x", listOf(Term(1, ZLong(1)), Term(0, ZLong(1))))

/** `(5x^2 + 3x + 7) + (x + 1)`, through `add-poly` directly. */
public fun sumOfPolynomials(): String {
    val table = freshTable()
    return either {
        show(addPoly(table, makeFiveXSquaredPlusThreeXPlusSeven(), makeXPlusOne()))
    }.getOrNull() ?: "unreachable"
}

/** `(5x^2 + 3x + 7) * (x + 1)`, through `mul-poly` directly. */
public fun productOfPolynomials(): String {
    val table = freshTable()
    return either {
        show(mulPoly(table, makeFiveXSquaredPlusThreeXPlusSeven(), makeXPlusOne()))
    }.getOrNull() ?: "unreachable"
}

public class S2_5_3SymbolicAlgebraTest :
    FunSpec({
        test("the book's own polynomial sums with a second polynomial under add-poly") {
            sumOfPolynomials() shouldBe "5*x^2 + 4*x + 8 in x"
        }

        test("the book's own polynomial multiplies with a second polynomial under mul-poly") {
            productOfPolynomials() shouldBe "5*x^3 + 8*x^2 + 10*x + 7 in x"
        }
    })
