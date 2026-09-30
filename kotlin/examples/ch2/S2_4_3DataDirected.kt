// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.4.3

package sicp.ch2.examples

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Datum
import sicp.runtime.DatumError
import sicp.runtime.DatumKey
import sicp.runtime.DatumOpTable
import sicp.runtime.PairCell
import sicp.runtime.Real
import sicp.runtime.Tagged

/** Build an immutable operation-table key from one or more type names. */
internal fun tagListKey(vararg tags: String): DatumKey =
    tags.foldRight(DatumKey.Empty as DatumKey) { tag, rest -> DatumKey.Pair(DatumKey.Symbol(tag), rest) }

/** Build a lookup key from already projected datum tags. */
internal fun keyTagList(tags: List<DatumKey>): DatumKey =
    tags.foldRight(DatumKey.Empty as DatumKey) { tag, rest -> DatumKey.Pair(tag, rest) }

/** Require exactly one argument for a selector handler. */
context(r: Raise<DatumError>)
internal fun sole(args: List<Datum>): Datum =
    args.singleOrNull() ?: r.raise(DatumError.BadDatum("selector handler requires one datum", args))

/** Extract the pair of real arguments used by a representation constructor. */
context(r: Raise<DatumError>)
internal fun realPair(args: List<Datum>): Pair<Double, Double> {
    if (args.size != 2) r.raise(DatumError.BadDatum("complex constructor requires two datums", args))
    return realOf(args[0]) to realOf(args[1])
}

/** Install the rectangular selectors and constructors in the host table. */
public fun installRectangularPackage(table: DatumOpTable) {
    val tag = tagListKey("rectangular")
    table.put(DatumKey.Symbol("real-part"), tag) { args -> rectRealPart(sole(args)) }
    table.put(DatumKey.Symbol("imag-part"), tag) { args -> rectImagPart(sole(args)) }
    table.put(DatumKey.Symbol("magnitude"), tag) { args -> rectMagnitude(sole(args)) }
    table.put(DatumKey.Symbol("angle"), tag) { args -> rectAngle(sole(args)) }
    table.put(DatumKey.Symbol("make-from-real-imag"), DatumKey.Symbol("rectangular")) { args ->
        val (x, y) = realPair(args)
        rectMakeFromRealImagTagged(x, y)
    }
    table.put(DatumKey.Symbol("make-from-mag-ang"), DatumKey.Symbol("rectangular")) { args ->
        val (mag, ang) = realPair(args)
        rectMakeFromMagAngTagged(mag, ang)
    }
}

/** Install the polar selectors and constructors in the host table. */
public fun installPolarPackage(table: DatumOpTable) {
    val tag = tagListKey("polar")
    table.put(DatumKey.Symbol("real-part"), tag) { args -> polarRealPart(sole(args)) }
    table.put(DatumKey.Symbol("imag-part"), tag) { args -> polarImagPart(sole(args)) }
    table.put(DatumKey.Symbol("magnitude"), tag) { args -> polarMagnitude(sole(args)) }
    table.put(DatumKey.Symbol("angle"), tag) { args -> polarAngle(sole(args)) }
    table.put(DatumKey.Symbol("make-from-real-imag"), DatumKey.Symbol("polar")) { args ->
        val (x, y) = realPair(args)
        polarMakeFromRealImagTagged(x, y)
    }
    table.put(DatumKey.Symbol("make-from-mag-ang"), DatumKey.Symbol("polar")) { args ->
        val (mag, ang) = realPair(args)
        polarMakeFromMagAngTagged(mag, ang)
    }
}

/**
 * Find an operation handler from the operation name and argument tags, then
 * apply it to the payload datums. An absent method is a typed data error.
 */
context(r: Raise<DatumError>)
public fun applyGeneric(
    table: DatumOpTable,
    op: String,
    args: List<Datum>,
): Datum {
    val tagNames = args.map { datum -> typeTag(datum) }
    val tags = tagNames.map { DatumKey.Symbol(it) }
    val handler =
        table.get(DatumKey.Symbol(op), keyTagList(tags))
            ?: r.raise(DatumError.BadDatum("No operation '$op' is installed for tags ${tagNames.joinToString()}", args))
    return r.handler(args.map { contents(it) })
}

context(r: Raise<DatumError>)
public fun realPart(
    table: DatumOpTable,
    z: Datum,
): Datum = applyGeneric(table, "real-part", listOf(z))

context(r: Raise<DatumError>)
public fun imagPart(
    table: DatumOpTable,
    z: Datum,
): Datum = applyGeneric(table, "imag-part", listOf(z))

context(r: Raise<DatumError>)
public fun magnitude(
    table: DatumOpTable,
    z: Datum,
): Datum = applyGeneric(table, "magnitude", listOf(z))

context(r: Raise<DatumError>)
public fun angle(
    table: DatumOpTable,
    z: Datum,
): Datum = applyGeneric(table, "angle", listOf(z))

/** Look up one representation constructor directly by its result tag. */
context(r: Raise<DatumError>)
public fun makeFromRealImag(
    table: DatumOpTable,
    x: Double,
    y: Double,
): Datum {
    val handler =
        table.get(DatumKey.Symbol("make-from-real-imag"), DatumKey.Symbol("rectangular"))
            ?: r.raise(DatumError.BadDatum("rectangular constructor is not installed"))
    return r.handler(listOf(Real(x), Real(y)))
}

context(r: Raise<DatumError>)
public fun makeFromMagAng(
    table: DatumOpTable,
    mag: Double,
    ang: Double,
): Datum {
    val handler =
        table.get(DatumKey.Symbol("make-from-mag-ang"), DatumKey.Symbol("polar"))
            ?: r.raise(DatumError.BadDatum("polar constructor is not installed"))
    return r.handler(listOf(Real(mag), Real(ang)))
}

public class S2_4_3DataDirectedTest :
    FunSpec({
        fun freshTable(): DatumOpTable {
            val table = DatumOpTable()
            installRectangularPackage(table)
            installPolarPackage(table)
            return table
        }

        test("an empty table reports an absent operation option") {
            val table = DatumOpTable()
            table.get(DatumKey.Symbol("real-part"), tagListKey("rectangular")) shouldBe null
        }

        test("representation constructors attach distinct tags to typed coordinate pairs") {
            val table = freshTable()
            val result =
                either {
                    makeFromRealImag(table, 3.0, 4.0) to makeFromMagAng(table, 1.0, 0.0)
                }
            val observations =
                result.fold(
                    { null },
                    { (rectangular, polar) ->
                        val rect = rectangular as Tagged
                        val polarData = polar as Tagged
                        val rectPayload = rect.payload as PairCell
                        val polarPayload = polarData.payload as PairCell
                        listOf(
                            rect.tag,
                            rectPayload.first,
                            rectPayload.second,
                            polarData.tag,
                            polarPayload.first,
                            polarPayload.second,
                        )
                    },
                )
            observations shouldBe listOf("rectangular", Real(3.0), Real(4.0), "polar", Real(1.0), Real(0.0))
        }

        test("generic selectors dispatch across rectangular and polar data") {
            val table = freshTable()
            val result =
                either {
                    val rectangular = makeFromRealImag(table, 3.0, 4.0)
                    val polar = makeFromMagAng(table, 5.0, 0.0)
                    Triple(magnitude(table, rectangular), realPart(table, polar), imagPart(table, polar))
                }
            result shouldBe Either.Right(Triple(Real(5.0), Real(5.0), Real(0.0)))
        }

        test("a missing operation-and-tag pair raises a typed error with both details") {
            val table = DatumOpTable()
            installRectangularPackage(table)
            val datum = attachTag("logarithmic", rectMakeFromRealImag(1.0, 1.0))
            val result = either { magnitude(table, datum) }
            val error = result.leftOrNull()
            (error is DatumError.BadDatum) shouldBe true
            error?.message shouldBe "No operation 'magnitude' is installed for tags logarithmic"
            error?.offending shouldBe listOf(datum)
        }
    })
