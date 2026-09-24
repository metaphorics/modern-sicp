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
 * 2.5.2's own worked example: an ordinary number and a complex number
 * cannot be added directly, since the table holds no `(integer,
 * complex)` handler. The book's fix is the coercion table -- a
 * procedure that views an ordinary number as a complex number with a
 * zero imaginary part, `scheme-number->complex`, installed under
 * `put-coercion` -- and the revised `apply-generic` that tries a
 * coercion on a miss before giving up. [installOrdinaryToComplex] is
 * that one coercion procedure; [addWithCoercion]/[addWithoutCoercion]
 * run the same addition through the revised and the plain dispatcher.
 */
private fun freshTable(): NumTable {
    val table = NumTable()
    installGenericArithmetic(table)
    return table
}

/** The book's `scheme-number->complex`: view an ordinary number as a
 * complex number whose imaginary part is zero. */
private fun ordinaryToComplex(z: Num): Num {
    val n = z as ZLong
    return Complex(Rect(n.n.toDouble(), 0.0))
}

/** Installs the one coercion procedure this example needs, the way the
 * book's `put-coercion` installs `scheme-number->complex`. */
public fun installOrdinaryToComplex(coercions: CoercionTable) {
    coercions.putCoercion(ZLong::class, Complex::class) { z -> ordinaryToComplex(z) }
}

/** Plain dispatch on an ordinary number and a complex number: no
 * `(integer, complex)` handler exists, so this misses with `NoMethod`,
 * exactly the gap 2.5.2 opens with. */
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
