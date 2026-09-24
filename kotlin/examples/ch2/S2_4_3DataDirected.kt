// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, section 2.4.3

package sicp.ch2.examples

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import sicp.runtime.Key
import sicp.runtime.OpTable
import sicp.runtime.SchemeError
import sicp.runtime.VReal
import sicp.runtime.VSym
import sicp.runtime.Value
import sicp.runtime.vlist

/**
 * The one-element tag-list key the book installs selectors under --
 * `(rectangular)` rather than the bare symbol `rectangular` -- to allow
 * for operations with multiple arguments, not all of the same type.
 * [Key.Nil]-terminated, so it composes with [applyGeneric]'s multi-tag
 * lookups too.
 */
internal fun tagListKey(vararg tags: String): Key = tags.foldRight(Key.Nil as Key) { tag, rest -> Key.Pair(Key.Sym(tag), rest) }

/** The tag-list key built from tags already looked up, for [applyGeneric]. */
internal fun keyTagList(tags: List<Key>): Key = tags.foldRight(Key.Nil as Key) { tag, rest -> Key.Pair(tag, rest) }

/** The single argument of a selector handler. */
context(r: Raise<SchemeError>)
internal fun sole(args: List<Value>): Value =
    args.singleOrNull() ?: r.raise(SchemeError.WrongArity("operation-table handler", "1", args.size))

/** The two real arguments of a constructor handler. */
context(r: Raise<SchemeError>)
internal fun realPair(args: List<Value>): Pair<Double, Double> {
    if (args.size != 2) r.raise(SchemeError.WrongArity("operation-table handler", "2", args.size))
    return realOf(args[0]) to realOf(args[1])
}

/**
 * Ben's package (2.4.3): the same internal procedures from 2.4.1 and the
 * same tagged constructors from 2.4.2, installed under the
 * operation-and-type table. No changes are necessary to interface them
 * to the rest of the system.
 */
public fun installRectangularPackage(table: OpTable) {
    val tag = tagListKey("rectangular")
    table.put(Key.Sym("real-part"), tag) { args -> rectRealPart(sole(args)) }
    table.put(Key.Sym("imag-part"), tag) { args -> rectImagPart(sole(args)) }
    table.put(Key.Sym("magnitude"), tag) { args -> rectMagnitude(sole(args)) }
    table.put(Key.Sym("angle"), tag) { args -> rectAngle(sole(args)) }
    table.put(Key.Sym("make-from-real-imag"), Key.Sym("rectangular")) { args ->
        val (x, y) = realPair(args)
        rectMakeFromRealImagTagged(x, y)
    }
    table.put(Key.Sym("make-from-mag-ang"), Key.Sym("rectangular")) { args ->
        val (mag, ang) = realPair(args)
        rectMakeFromMagAngTagged(mag, ang)
    }
}

/** Alyssa's package is analogous. */
public fun installPolarPackage(table: OpTable) {
    val tag = tagListKey("polar")
    table.put(Key.Sym("real-part"), tag) { args -> polarRealPart(sole(args)) }
    table.put(Key.Sym("imag-part"), tag) { args -> polarImagPart(sole(args)) }
    table.put(Key.Sym("magnitude"), tag) { args -> polarMagnitude(sole(args)) }
    table.put(Key.Sym("angle"), tag) { args -> polarAngle(sole(args)) }
    table.put(Key.Sym("make-from-real-imag"), Key.Sym("polar")) { args ->
        val (x, y) = realPair(args)
        polarMakeFromRealImagTagged(x, y)
    }
    table.put(Key.Sym("make-from-mag-ang"), Key.Sym("polar")) { args ->
        val (mag, ang) = realPair(args)
        polarMakeFromMagAngTagged(mag, ang)
    }
}

/**
 * Looks up the combination of the operation name and the argument tags
 * in the table, and applies the resulting handler to the untagged
 * contents: the book's `apply-generic`. A miss raises the book's own
 * message, naming the operation and the tags that had no handler --
 * the table's own answer to what the compiler cannot catch here that it
 * would catch for a closed `when`.
 */
context(r: Raise<SchemeError>)
public fun applyGeneric(
    table: OpTable,
    op: String,
    args: List<Value>,
): Value {
    val tagKeys = mutableListOf<Key>()
    val tagValues = mutableListOf<Value>()
    for (arg in args) {
        val tag = typeTag(arg)
        tagKeys += Key.Sym(tag)
        tagValues += VSym(tag)
    }
    val handler =
        table.get(Key.Sym(op), keyTagList(tagKeys))
            ?: r.raise(
                SchemeError.UserRaised(
                    "No method for these types: APPLY-GENERIC",
                    listOf(vlist(listOf(VSym(op), vlist(tagValues)))),
                ),
            )
    val bare = args.map { contents(it) }
    return r.handler(bare)
}

context(r: Raise<SchemeError>)
public fun realPart(
    table: OpTable,
    z: Value,
): Value = applyGeneric(table, "real-part", listOf(z))

context(r: Raise<SchemeError>)
public fun imagPart(
    table: OpTable,
    z: Value,
): Value = applyGeneric(table, "imag-part", listOf(z))

context(r: Raise<SchemeError>)
public fun magnitude(
    table: OpTable,
    z: Value,
): Value = applyGeneric(table, "magnitude", listOf(z))

context(r: Raise<SchemeError>)
public fun angle(
    table: OpTable,
    z: Value,
): Value = applyGeneric(table, "angle", listOf(z))

/**
 * Extracted straight from the table, since a constructor is always used
 * to make one particular type and so never needs [applyGeneric]'s
 * tag-based dispatch: the book constructs rectangular numbers from real
 * and imaginary parts, and polar numbers from magnitudes and angles.
 */
context(r: Raise<SchemeError>)
public fun makeFromRealImag(
    table: OpTable,
    x: Double,
    y: Double,
): Value {
    val handler =
        table.get(Key.Sym("make-from-real-imag"), Key.Sym("rectangular"))
            ?: r.raise(SchemeError.UserRaised("make-from-real-imag is not installed for rectangular", emptyList()))
    return r.handler(listOf(VReal(x), VReal(y)))
}

context(r: Raise<SchemeError>)
public fun makeFromMagAng(
    table: OpTable,
    mag: Double,
    ang: Double,
): Value {
    val handler =
        table.get(Key.Sym("make-from-mag-ang"), Key.Sym("polar"))
            ?: r.raise(SchemeError.UserRaised("make-from-mag-ang is not installed for polar", emptyList()))
    return r.handler(listOf(VReal(mag), VReal(ang)))
}

public class S2_4_3DataDirectedTest :
    FunSpec({
        fun freshTable(): OpTable {
            val table = OpTable()
            installRectangularPackage(table)
            installPolarPackage(table)
            return table
        }

        test("an empty table answers a miss with the absent option, never a false-ish sentinel") {
            val table = OpTable()
            table.get(Key.Sym("real-part"), tagListKey("rectangular")) shouldBe null
        }

        test("makeFromRealImag and makeFromMagAng build correctly tagged numbers") {
            val table = freshTable()
            val result =
                either {
                    makeFromRealImag(table, 3.0, 4.0).toString() to makeFromMagAng(table, 1.0, 0.0).toString()
                }
            result shouldBe Either.Right("(rectangular (3.0 . 4.0))" to "(polar (1.0 . 0.0))")
        }

        test("the generic selectors serve either representation") {
            val table = freshTable()
            val result =
                either {
                    val z = makeFromRealImag(table, 3.0, 4.0)
                    val w = makeFromMagAng(table, 5.0, 0.0)
                    Triple(magnitude(table, z), realPart(table, w), imagPart(table, w))
                }
            result shouldBe Either.Right(Triple(VReal(5.0) as Value, VReal(5.0) as Value, VReal(0.0) as Value))
        }

        test("a missing combination of operation and type raises the book's no-method error") {
            val table = OpTable()
            installRectangularPackage(table)
            val z = makeFromRealImagTaggedForTest(table)
            val result = either { magnitude(table, z) }
            result.leftOrNull().toString() shouldBe "No method for these types: APPLY-GENERIC (magnitude (logarithmic))"
        }
    })

/** A number tagged with a representation nothing in `table` installs, to probe [applyGeneric]'s miss path. */
private fun makeFromRealImagTaggedForTest(table: OpTable): Value = attachTag("logarithmic", rectMakeFromRealImag(1.0, 1.0))
