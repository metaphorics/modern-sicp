// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.4.1

package sicp.ch2.examples

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Datum
import sicp.runtime.DatumError
import sicp.runtime.DatumOpTable
import sicp.runtime.PairCell
import sicp.runtime.Real
import sicp.runtime.pair
import kotlin.math.atan2
import kotlin.math.cos
import kotlin.math.sin
import kotlin.math.sqrt

/** Read a real-valued complex component from the typed datum family. */
context(r: Raise<DatumError>)
internal fun realOf(value: Datum): Double =
    when (value) {
        is Real -> value.value
        else -> r.raise(DatumError.TypeMismatch("complex-number components are real datums", listOf(value)))
    }

/** Require the native pair representation used by the two complex packages. */
context(r: Raise<DatumError>)
private fun pairCell(value: Datum): PairCell =
    when (value) {
        is PairCell -> value
        else -> r.raise(DatumError.TypeMismatch("complex-number representation must be a pair", listOf(value)))
    }

/**
 * Rectangular coordinates store real and imaginary components in a native
 * pair cell; magnitude and angle follow from their trigonometric relations.
 */
context(r: Raise<DatumError>)
public fun rectRealPart(z: Datum): Datum = pairCell(z).first

context(r: Raise<DatumError>)
public fun rectImagPart(z: Datum): Datum = pairCell(z).second

context(r: Raise<DatumError>)
public fun rectMagnitude(z: Datum): Datum {
    val x = realOf(rectRealPart(z))
    val y = realOf(rectImagPart(z))
    return Real(sqrt(x * x + y * y))
}

context(r: Raise<DatumError>)
public fun rectAngle(z: Datum): Datum {
    val x = realOf(rectRealPart(z))
    val y = realOf(rectImagPart(z))
    return Real(atan2(y, x))
}

public fun rectMakeFromRealImag(
    x: Double,
    y: Double,
): Datum = pair(Real(x), Real(y))

public fun rectMakeFromMagAng(
    mag: Double,
    ang: Double,
): Datum = pair(Real(mag * cos(ang)), Real(mag * sin(ang)))

/** Polar coordinates store magnitude and angle in a native pair cell. */
context(r: Raise<DatumError>)
public fun polarMagnitude(z: Datum): Datum = pairCell(z).first

context(r: Raise<DatumError>)
public fun polarAngle(z: Datum): Datum = pairCell(z).second

context(r: Raise<DatumError>)
public fun polarRealPart(z: Datum): Datum {
    val mag = realOf(polarMagnitude(z))
    val ang = realOf(polarAngle(z))
    return Real(mag * cos(ang))
}

context(r: Raise<DatumError>)
public fun polarImagPart(z: Datum): Datum {
    val mag = realOf(polarMagnitude(z))
    val ang = realOf(polarAngle(z))
    return Real(mag * sin(ang))
}

public fun polarMakeFromRealImag(
    x: Double,
    y: Double,
): Datum = pair(Real(sqrt(x * x + y * y)), Real(atan2(y, x)))

public fun polarMakeFromMagAng(
    mag: Double,
    ang: Double,
): Datum = pair(Real(mag), Real(ang))

/** Combine representations through the generic selectors and constructors. */
context(r: Raise<DatumError>)
public fun addComplex(
    table: DatumOpTable,
    z1: Datum,
    z2: Datum,
): Datum {
    val real = realOf(realPart(table, z1)) + realOf(realPart(table, z2))
    val imaginary = realOf(imagPart(table, z1)) + realOf(imagPart(table, z2))
    return makeFromRealImag(table, real, imaginary)
}

context(r: Raise<DatumError>)
public fun subComplex(
    table: DatumOpTable,
    z1: Datum,
    z2: Datum,
): Datum {
    val real = realOf(realPart(table, z1)) - realOf(realPart(table, z2))
    val imaginary = realOf(imagPart(table, z1)) - realOf(imagPart(table, z2))
    return makeFromRealImag(table, real, imaginary)
}

context(r: Raise<DatumError>)
public fun mulComplex(
    table: DatumOpTable,
    z1: Datum,
    z2: Datum,
): Datum {
    val mag = realOf(magnitude(table, z1)) * realOf(magnitude(table, z2))
    val ang = realOf(angle(table, z1)) + realOf(angle(table, z2))
    return makeFromMagAng(table, mag, ang)
}

context(r: Raise<DatumError>)
public fun divComplex(
    table: DatumOpTable,
    z1: Datum,
    z2: Datum,
): Datum {
    val mag = realOf(magnitude(table, z1)) / realOf(magnitude(table, z2))
    val ang = realOf(angle(table, z1)) - realOf(angle(table, z2))
    return makeFromMagAng(table, mag, ang)
}

public class S2_4_1ComplexArithmeticTest :
    FunSpec({
        fun freshTable(): DatumOpTable {
            val table = DatumOpTable()
            installRectangularPackage(table)
            installPolarPackage(table)
            return table
        }

        test("rectangular selectors and construction use two real-valued fields") {
            val result =
                either {
                    val z = rectMakeFromRealImag(3.0, 4.0)
                    Triple(realOf(rectRealPart(z)), realOf(rectImagPart(z)), realOf(rectMagnitude(z)))
                }
            result shouldBe Either.Right(Triple(3.0, 4.0, 5.0))
        }

        test("polar selectors and construction expose magnitude, angle, and real part") {
            val result =
                either {
                    val z = polarMakeFromMagAng(5.0, 0.0)
                    Triple(realOf(polarMagnitude(z)), realOf(polarAngle(z)), realOf(polarRealPart(z)))
                }
            result shouldBe Either.Right(Triple(5.0, 0.0, 5.0))
        }

        test("addition works for rectangular, polar, and mixed inputs") {
            val table = freshTable()
            val result =
                either {
                    val z1 = makeFromRealImag(table, 3.0, 4.0)
                    val z2 = makeFromMagAng(table, 1.0, 0.0)
                    val sum = addComplex(table, z1, z2)
                    realOf(realPart(table, sum)) to realOf(imagPart(table, sum))
                }
            result shouldBe Either.Right(4.0 to 4.0)
        }

        test("subtraction, multiplication, and division follow their component formulas") {
            val table = freshTable()
            val result =
                either {
                    val z1 = makeFromRealImag(table, 3.0, 4.0)
                    val z2 = makeFromMagAng(table, 1.0, 0.0)
                    val difference = subComplex(table, z1, z2)
                    val differenceParts = realOf(realPart(table, difference)) to realOf(imagPart(table, difference))
                    val product = mulComplex(table, z1, z2)
                    val productParts = realOf(magnitude(table, product)) to realOf(angle(table, product))
                    val quotient = divComplex(table, z1, z2)
                    val quotientParts = realOf(magnitude(table, quotient)) to realOf(angle(table, quotient))
                    val z1Angle = realOf(angle(table, z1))
                    Triple(differenceParts, productParts, quotientParts) to z1Angle
                }
            result shouldBe Either.Right(Triple(2.0 to 4.0, 5.0 to 0.9272952180016122, 5.0 to 0.9272952180016122) to 0.9272952180016122)
        }
    })
