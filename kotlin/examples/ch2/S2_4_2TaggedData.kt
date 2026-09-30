// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.4.2

package sicp.ch2.examples

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Datum
import sicp.runtime.DatumError
import sicp.runtime.PairCell
import sicp.runtime.Real
import sicp.runtime.Tagged
import sicp.runtime.Whole

/** Attach a representation name to a native datum payload. */
public fun attachTag(
    typeTag: String,
    contents: Datum,
): Datum = Tagged(typeTag, contents)

/** Read the representation name from a tagged datum. */
context(r: Raise<DatumError>)
public fun typeTag(datum: Datum): String =
    when (datum) {
        is Tagged -> datum.tag
        else -> r.raise(DatumError.BadDatum("datum has no representation tag", listOf(datum)))
    }

/** Read the payload without changing its underlying pair structure. */
context(r: Raise<DatumError>)
public fun contents(datum: Datum): Datum =
    when (datum) {
        is Tagged -> datum.payload
        else -> r.raise(DatumError.BadDatum("datum has no tagged payload", listOf(datum)))
    }

/** Recognize rectangular representation data without raising on other values. */
public fun isRectangular(z: Datum): Boolean = z is Tagged && z.tag == "rectangular"

/** Recognize polar representation data without raising on other values. */
public fun isPolar(z: Datum): Boolean = z is Tagged && z.tag == "polar"

/** Attach the rectangular tag to either rectangular construction. */
public fun rectMakeFromRealImagTagged(
    x: Double,
    y: Double,
): Datum = attachTag("rectangular", rectMakeFromRealImag(x, y))

public fun rectMakeFromMagAngTagged(
    mag: Double,
    ang: Double,
): Datum = attachTag("rectangular", rectMakeFromMagAng(mag, ang))

/** Attach the polar tag to either polar construction. */
public fun polarMakeFromRealImagTagged(
    x: Double,
    y: Double,
): Datum = attachTag("polar", polarMakeFromRealImag(x, y))

public fun polarMakeFromMagAngTagged(
    mag: Double,
    ang: Double,
): Datum = attachTag("polar", polarMakeFromMagAng(mag, ang))

/** Dispatch a complex selector on the representation tag. */
context(r: Raise<DatumError>)
public fun realPartDispatch(z: Datum): Datum =
    when {
        isRectangular(z) -> rectRealPart(contents(z))
        isPolar(z) -> polarRealPart(contents(z))
        else -> r.raise(DatumError.BadDatum("unknown real-part representation", listOf(z)))
    }

context(r: Raise<DatumError>)
public fun imagPartDispatch(z: Datum): Datum =
    when {
        isRectangular(z) -> rectImagPart(contents(z))
        isPolar(z) -> polarImagPart(contents(z))
        else -> r.raise(DatumError.BadDatum("unknown imaginary-part representation", listOf(z)))
    }

context(r: Raise<DatumError>)
public fun magnitudeDispatch(z: Datum): Datum =
    when {
        isRectangular(z) -> rectMagnitude(contents(z))
        isPolar(z) -> polarMagnitude(contents(z))
        else -> r.raise(DatumError.BadDatum("unknown magnitude representation", listOf(z)))
    }

context(r: Raise<DatumError>)
public fun angleDispatch(z: Datum): Datum =
    when {
        isRectangular(z) -> rectAngle(contents(z))
        isPolar(z) -> polarAngle(contents(z))
        else -> r.raise(DatumError.BadDatum("unknown angle representation", listOf(z)))
    }

public class S2_4_2TaggedDataTest :
    FunSpec({
        test("rectangular data preserves its tag and coordinate pair") {
            val z = rectMakeFromRealImagTagged(3.0, 4.0) as Tagged
            z.tag shouldBe "rectangular"
            val coordinates = z.payload as PairCell
            coordinates.first shouldBe Real(3.0)
            coordinates.second shouldBe Real(4.0)
            z.toString() shouldBe
                "Tagged(tag=\"rectangular\", payload=PairCell(first=Real(value=3.0), second=Real(value=4.0)))"
        }

        test("polar data preserves its tag and magnitude-angle pair") {
            val z = polarMakeFromMagAngTagged(1.0, 0.0) as Tagged
            z.tag shouldBe "polar"
            val coordinates = z.payload as PairCell
            coordinates.first shouldBe Real(1.0)
            coordinates.second shouldBe Real(0.0)
        }

        test("generic dispatch returns components and removes the representation tag") {
            val result =
                either {
                    val rectangular = rectMakeFromRealImagTagged(3.0, 4.0)
                    val polar = polarMakeFromMagAngTagged(5.0, 0.0)
                    Triple(
                        realOf(magnitudeDispatch(rectangular)),
                        realOf(realPartDispatch(polar)),
                        isRectangular(rectangular) to isPolar(rectangular),
                    )
                }
            result shouldBe Either.Right(Triple(5.0, 5.0, true to false))
        }

        test("an untagged complex datum raises a typed dispatch error") {
            val datum = rectMakeFromRealImag(3.0, 4.0)
            val result = either { realPartDispatch(datum) }
            val error = result.leftOrNull()
            (error is DatumError.BadDatum) shouldBe true
            error?.offending shouldBe listOf(datum)
        }

        test("a non-tagged scalar raises a typed tag-access error") {
            val datum = Whole(1L)
            val result = either { typeTag(datum) }
            val error = result.leftOrNull()
            (error is DatumError.BadDatum) shouldBe true
            error?.offending shouldBe listOf(datum)
        }
    })
