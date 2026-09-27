// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.4.1

package sicp.ch2.examples

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.OpTable
import sicp.runtime.SchemeError
import sicp.runtime.VPair
import sicp.runtime.VReal
import sicp.runtime.Value
import sicp.runtime.car
import sicp.runtime.cdr
import kotlin.math.atan2
import kotlin.math.cos
import kotlin.math.sin
import kotlin.math.sqrt

/**
 * The real component of a complex-number part: the section's magnitudes,
 * angles, and rectangular coordinates are all reals.
 */
context(r: Raise<SchemeError>)
internal fun realOf(v: Value): Double =
    when (v) {
        is VReal -> v.d
        else -> r.raise(SchemeError.TypeMismatch("complex-number components are reals: $v"))
    }

/**
 * Ben Bitdiddle's rectangular representation (2.4.1): a complex number is
 * the pair (real part, imaginary part). Selecting the real and imaginary
 * parts is direct, as is constructing one from them; the magnitude and
 * angle need the trigonometric relations relating `(x, y)` to `(r, A)`.
 */
context(r: Raise<SchemeError>)
public fun rectRealPart(z: Value): Value = car(z)

context(r: Raise<SchemeError>)
public fun rectImagPart(z: Value): Value = cdr(z)

context(r: Raise<SchemeError>)
public fun rectMagnitude(z: Value): Value {
    val x = realOf(rectRealPart(z))
    val y = realOf(rectImagPart(z))
    return VReal(sqrt(x * x + y * y))
}

context(r: Raise<SchemeError>)
public fun rectAngle(z: Value): Value {
    val x = realOf(rectRealPart(z))
    val y = realOf(rectImagPart(z))
    return VReal(atan2(y, x))
}

public fun rectMakeFromRealImag(
    x: Double,
    y: Double,
): Value = VPair(VReal(x), VReal(y))

public fun rectMakeFromMagAng(
    mag: Double,
    ang: Double,
): Value = VPair(VReal(mag * cos(ang)), VReal(mag * sin(ang)))

/**
 * Alyssa P. Hacker's polar representation (2.4.1): a complex number is
 * the pair (magnitude, angle). Selecting the magnitude and angle is
 * direct, as is constructing one from them; the real and imaginary parts
 * need the same trigonometric relations run the other way.
 */
context(r: Raise<SchemeError>)
public fun polarMagnitude(z: Value): Value = car(z)

context(r: Raise<SchemeError>)
public fun polarAngle(z: Value): Value = cdr(z)

context(r: Raise<SchemeError>)
public fun polarRealPart(z: Value): Value {
    val mag = realOf(polarMagnitude(z))
    val ang = realOf(polarAngle(z))
    return VReal(mag * cos(ang))
}

context(r: Raise<SchemeError>)
public fun polarImagPart(z: Value): Value {
    val mag = realOf(polarMagnitude(z))
    val ang = realOf(polarAngle(z))
    return VReal(mag * sin(ang))
}

public fun polarMakeFromRealImag(
    x: Double,
    y: Double,
): Value = VPair(VReal(sqrt(x * x + y * y)), VReal(atan2(y, x)))

public fun polarMakeFromMagAng(
    mag: Double,
    ang: Double,
): Value = VPair(VReal(mag), VReal(ang))

/**
 * The complex-arithmetic operations, written once against the generic
 * selectors and constructors of 2.4.3 ([realPart], [imagPart],
 * [magnitude], [angle], [makeFromRealImag], [makeFromMagAng]). The book
 * states these four here in 2.4.1 against four selectors and two
 * constructors it simply assumes exist, so this file's examples only run
 * once [installRectangularPackage] and [installPolarPackage] have
 * populated the table those generic names dispatch through.
 */
context(r: Raise<SchemeError>)
public fun addComplex(
    table: OpTable,
    z1: Value,
    z2: Value,
): Value {
    val re = realOf(realPart(table, z1)) + realOf(realPart(table, z2))
    val im = realOf(imagPart(table, z1)) + realOf(imagPart(table, z2))
    return makeFromRealImag(table, re, im)
}

context(r: Raise<SchemeError>)
public fun subComplex(
    table: OpTable,
    z1: Value,
    z2: Value,
): Value {
    val re = realOf(realPart(table, z1)) - realOf(realPart(table, z2))
    val im = realOf(imagPart(table, z1)) - realOf(imagPart(table, z2))
    return makeFromRealImag(table, re, im)
}

context(r: Raise<SchemeError>)
public fun mulComplex(
    table: OpTable,
    z1: Value,
    z2: Value,
): Value {
    val mag = realOf(magnitude(table, z1)) * realOf(magnitude(table, z2))
    val ang = realOf(angle(table, z1)) + realOf(angle(table, z2))
    return makeFromMagAng(table, mag, ang)
}

context(r: Raise<SchemeError>)
public fun divComplex(
    table: OpTable,
    z1: Value,
    z2: Value,
): Value {
    val mag = realOf(magnitude(table, z1)) / realOf(magnitude(table, z2))
    val ang = realOf(angle(table, z1)) - realOf(angle(table, z2))
    return makeFromMagAng(table, mag, ang)
}

public class S2_4_1ComplexArithmeticTest :
    FunSpec({
        fun freshTable(): OpTable {
            val table = OpTable()
            installRectangularPackage(table)
            installPolarPackage(table)
            return table
        }

        test("Ben's rectangular selectors and constructor work directly on an untagged pair") {
            val result =
                either {
                    val z = rectMakeFromRealImag(3.0, 4.0)
                    Triple(rectRealPart(z), rectImagPart(z), rectMagnitude(z))
                }
            result shouldBe Either.Right(Triple(VReal(3.0) as Value, VReal(4.0) as Value, VReal(5.0) as Value))
        }

        test("Alyssa's polar selectors and constructor work directly on an untagged pair") {
            val result =
                either {
                    val z = polarMakeFromMagAng(5.0, 0.0)
                    Triple(polarMagnitude(z), polarAngle(z), polarRealPart(z))
                }
            result shouldBe Either.Right(Triple(VReal(5.0) as Value, VReal(0.0) as Value, VReal(5.0) as Value))
        }

        test("addComplex works whether the arguments are rectangular, polar, or a mix") {
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

        test("subComplex, mulComplex, and divComplex agree with the book's formulas") {
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
