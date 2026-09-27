// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.4.2

package sicp.ch2.examples

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.SchemeError
import sicp.runtime.VInt
import sicp.runtime.VReal
import sicp.runtime.VTagged
import sicp.runtime.Value

/**
 * Tags `contents` with `typeTag`: the book's `attach-tag`. The runtime's
 * [VTagged] already is the tag-and-contents pairing; this is the named
 * constructor the section's listings call.
 */
public fun attachTag(
    typeTag: String,
    contents: Value,
): Value = VTagged(typeTag, contents)

/** The tag of a tagged datum: the book's `type-tag`. */
context(r: Raise<SchemeError>)
public fun typeTag(datum: Value): String =
    when (datum) {
        is VTagged -> datum.tag
        else -> r.raise(SchemeError.UserRaised("Bad tagged datum: TYPE-TAG", listOf(datum)))
    }

/** The contents of a tagged datum: the book's `contents`. */
context(r: Raise<SchemeError>)
public fun contents(datum: Value): Value =
    when (datum) {
        is VTagged -> datum.data
        else -> r.raise(SchemeError.UserRaised("Bad tagged datum: CONTENTS", listOf(datum)))
    }

/** Recognizes rectangular numbers: the book's `rectangular?`. A datum
 * with no tag at all is simply neither, not an error. */
public fun isRectangular(z: Value): Boolean = z is VTagged && z.tag == "rectangular"

/** Recognizes polar numbers: the book's `polar?`. */
public fun isPolar(z: Value): Boolean = z is VTagged && z.tag == "polar"

/** Ben's tagged constructors: identical to [rectMakeFromRealImag] and
 * [rectMakeFromMagAng], except that they attach the tag. */
public fun rectMakeFromRealImagTagged(
    x: Double,
    y: Double,
): Value = attachTag("rectangular", rectMakeFromRealImag(x, y))

public fun rectMakeFromMagAngTagged(
    mag: Double,
    ang: Double,
): Value = attachTag("rectangular", rectMakeFromMagAng(mag, ang))

/** Alyssa's tagged constructors. */
public fun polarMakeFromRealImagTagged(
    x: Double,
    y: Double,
): Value = attachTag("polar", polarMakeFromRealImag(x, y))

public fun polarMakeFromMagAngTagged(
    mag: Double,
    ang: Double,
): Value = attachTag("polar", polarMakeFromMagAng(mag, ang))

/**
 * Each generic selector dispatches explicitly on the tag: the book's
 * `real-part`, `imag-part`, `magnitude`, and `angle` of 2.4.2, in terms
 * of Ben's and Alyssa's untagged selectors from 2.4.1. Section 2.4.3
 * replaces this `when` with a table lookup, keeping the same names; both
 * versions live in this section under distinct ones.
 */
context(r: Raise<SchemeError>)
public fun realPartDispatch(z: Value): Value =
    when {
        isRectangular(z) -> rectRealPart(contents(z))
        isPolar(z) -> polarRealPart(contents(z))
        else -> r.raise(SchemeError.UserRaised("Unknown type: REAL-PART", listOf(z)))
    }

context(r: Raise<SchemeError>)
public fun imagPartDispatch(z: Value): Value =
    when {
        isRectangular(z) -> rectImagPart(contents(z))
        isPolar(z) -> polarImagPart(contents(z))
        else -> r.raise(SchemeError.UserRaised("Unknown type: IMAG-PART", listOf(z)))
    }

context(r: Raise<SchemeError>)
public fun magnitudeDispatch(z: Value): Value =
    when {
        isRectangular(z) -> rectMagnitude(contents(z))
        isPolar(z) -> polarMagnitude(contents(z))
        else -> r.raise(SchemeError.UserRaised("Unknown type: MAGNITUDE", listOf(z)))
    }

context(r: Raise<SchemeError>)
public fun angleDispatch(z: Value): Value =
    when {
        isRectangular(z) -> rectAngle(contents(z))
        isPolar(z) -> polarAngle(contents(z))
        else -> r.raise(SchemeError.UserRaised("Unknown type: ANGLE", listOf(z)))
    }

public class S2_4_2TaggedDataTest :
    FunSpec({
        test("a tagged rectangular number prints its tag and its untagged contents") {
            val z = rectMakeFromRealImagTagged(3.0, 4.0)
            z.toString() shouldBe "(rectangular (3.0 . 4.0))"
        }

        test("a tagged polar number prints its tag and its untagged contents") {
            val z = polarMakeFromMagAngTagged(1.0, 0.0)
            z.toString() shouldBe "(polar (1.0 . 0.0))"
        }

        test("the dispatch selectors serve either tag, stripping and never leaking the tag") {
            val result =
                either {
                    val rect = rectMakeFromRealImagTagged(3.0, 4.0)
                    val polar = polarMakeFromMagAngTagged(5.0, 0.0)
                    Triple(magnitudeDispatch(rect), realPartDispatch(polar), isRectangular(rect) to isPolar(rect))
                }
            result shouldBe Either.Right(Triple(VReal(5.0) as Value, VReal(5.0) as Value, true to false))
        }

        test("an untagged datum raises the book's unknown-type error") {
            val z = rectMakeFromRealImag(3.0, 4.0)
            val result = either { realPartDispatch(z) }
            result shouldBe Either.Left(SchemeError.UserRaised("Unknown type: REAL-PART", listOf(z)))
        }

        test("a non-tagged, non-pair datum raises the book's bad-tagged-datum error") {
            val notTagged = VInt(1)
            val result = either { typeTag(notTagged) }
            result shouldBe Either.Left(SchemeError.UserRaised("Bad tagged datum: TYPE-TAG", listOf(notTagged)))
        }
    })
