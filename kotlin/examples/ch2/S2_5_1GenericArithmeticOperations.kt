// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.5.1: generic arithmetic operations

package sicp.ch2.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch2.exercises.Complex
import sicp.ch2.exercises.NumTable
import sicp.ch2.exercises.Rect
import sicp.ch2.exercises.ZLong
import sicp.ch2.exercises.applyGeneric
import sicp.ch2.exercises.installGenericArithmetic
import sicp.ch2.exercises.qr
import sicp.ch2.exercises.show

/**
 * 2.5.1 installs a single generic front door -- `add`, `sub`, `mul`,
 * `div` -- that dispatches on its arguments' tags to whichever package
 * (ordinary, rational, or complex) can handle them, exactly as
 * [installGenericArithmetic] wires the three packages into one table.
 * Any individual package may still be used directly by procedures
 * designed for its own representation, but a caller going through the
 * generic operations never needs to know which package answered.
 */
private fun freshTable(): NumTable {
    val table = NumTable()
    installGenericArithmetic(table)
    return table
}

/** The book's own Figure 2.24 number, `3 + 4i`, added to itself through
 * the generic front door rather than the complex package's own
 * `add-complex` by name. */
public fun addTwoComplexes(): String {
    val table = freshTable()
    return either {
        show(applyGeneric(table, "add", listOf(Complex(Rect(3.0, 4.0)), Complex(Rect(3.0, 4.0)))))
    }.getOrNull() ?: "unreachable"
}

/** Two ordinary numbers added through the same generic `add` the
 * complex numbers above went through. */
public fun addTwoOrdinary(): String {
    val table = freshTable()
    return either {
        show(applyGeneric(table, "add", listOf(ZLong(3), ZLong(4))))
    }.getOrNull() ?: "unreachable"
}

/** Two rationals added through the generic front door; the rational
 * package reduces the sum to lowest terms the same way it always does. */
public fun addTwoRationals(): String {
    val table = freshTable()
    return either {
        show(applyGeneric(table, "add", listOf(qr(1, 2), qr(1, 3))))
    }.getOrNull() ?: "unreachable"
}

public class S2_5_1GenericArithmeticOperationsTest :
    FunSpec({
        test("the generic add answers ordinary, rational, and complex arguments alike") {
            addTwoOrdinary() shouldBe "7"
            addTwoRationals() shouldBe "5/6"
            addTwoComplexes() shouldBe "6.0+8.0i"
        }
    })
