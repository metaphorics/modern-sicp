// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.5.2: combining data of different types

package sicp.ch2.examples

import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.ch2.exercises.CoercionTable
import sicp.ch2.exercises.Complex
import sicp.ch2.exercises.GenError
import sicp.ch2.exercises.Num
import sicp.ch2.exercises.NumTable
import sicp.ch2.exercises.Rect
import sicp.ch2.exercises.ZLong
import sicp.ch2.exercises.applyGeneric
import sicp.ch2.exercises.applyGenericWithCoercion
import sicp.ch2.exercises.installGenericArithmetic
import sicp.ch2.exercises.show

/**
 * An ordinary numeric value cannot be added directly to a complex value
 * when no mixed-type handler exists. The coercion-table repair converts an
 * ordinary integer into a rectangular complex value with zero imaginary
 * component. [installOrdinaryToComplex] installs that conversion;
 * [addWithCoercion] and [addWithoutCoercion] compare dispatch with and
 * without the installed conversion.
 */
private fun freshTable(): NumTable {
    val table = NumTable()
    installGenericArithmetic(table)
    return table
}

/** Convert an ordinary integer value to a rectangular complex value. */
private fun ordinaryToComplex(z: Num): Num {
    val n = z as ZLong
    return Complex(Rect(n.n.toDouble(), 0.0))
}

/** Install the one numeric-to-complex conversion this example requires. */
public fun installOrdinaryToComplex(coercions: CoercionTable) {
    coercions.putCoercion(ZLong::class, Complex::class) { z -> ordinaryToComplex(z) }
}

/** Plain dispatch has no handler for an ordinary number paired with a complex value. */
public fun addWithoutCoercion(): Boolean {
    val table = freshTable()
    val result = either { applyGeneric(table, "add", listOf(ZLong(3), Complex(Rect(2.0, 4.0)))) }
    return result.leftOrNull() is GenError.NoMethod
}

/** The same addition through the revised dispatcher: the ordinary
 * number coerces to a complex number with zero imaginary part, `3` to
 * `3.0+0.0i`, and the addition proceeds in the complex package. */
public fun addWithCoercion(): String {
    val table = freshTable()
    val coercions = CoercionTable()
    installOrdinaryToComplex(coercions)
    return either {
        show(applyGenericWithCoercion(table, coercions, "add", listOf(ZLong(3), Complex(Rect(2.0, 4.0)))))
    }.getOrNull() ?: "unreachable"
}

public class S2_5_2CombiningDataOfDifferentTypesTest :
    FunSpec({
        test("plain dispatch misses an ordinary number added to a complex number") {
            addWithoutCoercion() shouldBe true
        }

        test("the coercion table lets the same addition through") {
            addWithCoercion() shouldBe "5.0+4.0i"
        }
    })
