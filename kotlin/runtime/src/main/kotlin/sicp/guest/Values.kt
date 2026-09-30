// SPDX-License-Identifier: GPL-3.0-only
package sicp.guest

import arrow.core.raise.Raise

/** The host operation shape of every invokable value: given argument values
 * it yields one value or raises a [GuestError]. */
public typealias Primitive = Raise<GuestError>.(List<GValue>) -> GValue

/** The lazy module's finite forcing instrument counters (section 4.1). */
public class ForceCounters {
    public var attempts: Long = 0
    public var computations: Long = 0
}

/**
 * Guest runtime values. Scalars are inline classes; every shape that can
 * participate in aliasing or object cycles is a class, so `===` and the
 * capture rules of sections 3.4/3.5 behave exactly as written.
 */
public sealed interface GValue {
    @JvmInline public value class VInt(
        public val value: Int,
    ) : GValue

    @JvmInline public value class VLong(
        public val value: Long,
    ) : GValue

    @JvmInline public value class VDouble(
        public val value: Double,
    ) : GValue

    @JvmInline public value class VBool(
        public val value: Boolean,
    ) : GValue

    @JvmInline public value class VString(
        public val value: String,
    ) : GValue

    public data object VNull : GValue

    public data object VUnit : GValue

    /** A read of a binding before its initializer ran (3.8 UnassignedRead). */
    public data object VUnassigned : GValue

    /** Lists carry mutability and set shape: reads through a read-only view
     * still observe writes through a mutable alias (3.5). */
    public class VList(
        public val items: MutableList<GValue>,
        public val mutable: Boolean,
        public val asSet: Boolean = false,
    ) : GValue

    public class VMap(
        public val entries: LinkedHashMap<GValue, GValue>,
        public val mutable: Boolean,
    ) : GValue

    public class VPair(
        public val first: GValue,
        public val second: GValue,
    ) : GValue

    /** A data-class, data-object, or plain-class instance; [structural]
     * marks data, whose `==` compares fields. Object cycles are legal
     * through [fields]. */
    public class VObject(
        public val className: String,
        public val structural: Boolean,
        public val fields: MutableMap<String, GValue>,
    ) : GValue

    /** A function value: user closures and surface primitives share this one
     * application shape. [delays] marks lazy-module delayed parameters. */
    public class VFunction(
        public val name: String?,
        public val arity: Int,
        public val delays: List<Boolean> = emptyList(),
        public val apply: Primitive,
    ) : GValue

    /** A memoized thunk. [transparent] marks lazy-module parameter delays,
     * which force on reference; explicit `Thunk<T>` values do not. */
    public class VThunk(
        public var state: ThunkState,
        public val transparent: Boolean = false,
        public val counters: ForceCounters? = null,
    ) : GValue

    /** A lazy list node: an eager head and a memoized tail thunk (4.2.3). */
    public class VLazyList(
        public val head: GValue,
        public val tail: VThunk,
    ) : GValue

    /** Engine carrier: checked syntax riding in machine registers. */
    public class VNode(
        public val node: Node,
    ) : GValue

    /** Engine carrier: an evaluator environment in machine registers. */
    public class VEnvVal(
        public val env: Env,
    ) : GValue

    /** The seeded generator of the search module. */
    public class VRandom(
        public var seed: Long,
    ) : GValue
}

/** One admitted literal lowered at its checked width, not merely at the
 * spelling's suffix. Kotlin can infer an unsuffixed integer as Long from an
 * expected parameter/property/result type. The checker has already proved
 * range and type before this conversion runs. */
public fun checkedLiteralValue(
    literal: Literal,
    type: GuestType?,
): GValue =
    when (literal.kind) {
        LiteralKind.INT, LiteralKind.LONG -> {
            if (literal.kind == LiteralKind.LONG || (type as? GuestType.Named)?.name == "Long") {
                GValue.VLong(literal.text.toLong())
            } else {
                GValue.VInt(literal.text.toInt())
            }
        }

        LiteralKind.DOUBLE -> {
            GValue.VDouble(literal.text.toDouble())
        }

        LiteralKind.BOOLEAN -> {
            GValue.VBool(literal.text == "true")
        }

        LiteralKind.STRING -> {
            GValue.VString(literal.text)
        }

        LiteralKind.NULL -> {
            GValue.VNull
        }
    }

public sealed interface ThunkState {
    public class Delayed(
        public val compute: Primitive,
    ) : ThunkState

    public class Forced(
        public val value: GValue,
    ) : ThunkState
}

/** One mutable binding slot: closures capture the slot, never a copy (3.4). */
public class Cell(
    public var value: GValue,
)

/** Lexical scope as a chain of frames of capture-shared cells. */
public class Env(
    public val bindings: MutableMap<String, Cell>,
    public val parent: Env?,
) {
    public fun lookup(name: String): Cell? {
        var here: Env? = this
        while (here != null) {
            val hit = here.bindings[name]
            if (hit != null) return hit
            here = here.parent
        }
        return null
    }

    public fun define(
        name: String,
        value: GValue,
    ) {
        bindings[name] = Cell(value)
    }

    public companion object {
        public fun root(): Env = Env(mutableMapOf(), null)

        public fun child(parent: Env): Env = Env(mutableMapOf(), parent)
    }
}

/** Section 3.7 rendering for printed and interpolated values; null when the
 * value is not one of the four printable shapes. */
public fun renderPrinted(value: GValue): String? =
    when (value) {
        is GValue.VLong -> value.value.toString()
        is GValue.VDouble -> value.value.toString()
        is GValue.VBool -> value.value.toString()
        is GValue.VString -> value.value
        else -> null
    }

/** Structural `==` of sections 3.5 and 2.4: data classes, pairs, lists, and
 * maps compare contents; closures, thunks, and plain-class instances are
 * identical only to themselves. */
context(r: Raise<GuestError>)
public fun valueEquals(
    a: GValue,
    b: GValue,
): Boolean {
    if (a === b) return true
    return valueEqualsSeen(a, b, HashSet())
}

context(r: Raise<GuestError>)
private fun valueEqualsSeen(
    a: GValue,
    b: GValue,
    seen: MutableSet<Pair<GValue, GValue>>,
): Boolean =
    when {
        a is GValue.VInt && b is GValue.VInt -> {
            a.value == b.value
        }

        a is GValue.VLong && b is GValue.VLong -> {
            a.value == b.value
        }

        a is GValue.VDouble && b is GValue.VDouble -> {
            a.value == b.value
        }

        a is GValue.VBool && b is GValue.VBool -> {
            a.value == b.value
        }

        a is GValue.VString && b is GValue.VString -> {
            a.value == b.value
        }

        a is GValue.VPair && b is GValue.VPair -> {
            guardSeen(a, b, seen) && valueEqualsSeen(a.first, b.first, seen) &&
                valueEqualsSeen(a.second, b.second, seen)
        }

        a is GValue.VList && b is GValue.VList -> {
            a.items.size == b.items.size && guardSeen(a, b, seen) && a.items.zip(b.items).all { (x, y) -> valueEqualsSeen(x, y, seen) }
        }

        a is GValue.VLazyList || b is GValue.VLazyList -> {
            valueEqualsSeen(materialize(a), materialize(b), seen)
        }

        a is GValue.VMap && b is GValue.VMap -> {
            mapEquals(a, b)
        }

        a is GValue.VObject && b is GValue.VObject -> {
            a.structural && b.structural && a.className == b.className &&
                a.fields.size == b.fields.size &&
                guardSeen(a, b, seen) && a.fields.all { (name, x) -> b.fields[name]?.let { valueEqualsSeen(x, it, seen) } == true }
        }

        a is GValue.VNull && b is GValue.VNull -> {
            true
        }

        a is GValue.VUnit && b is GValue.VUnit -> {
            true
        }

        a is GValue.VUnassigned && b is GValue.VUnassigned -> {
            true
        }

        else -> {
            a === b
        }
    }

private fun guardSeen(
    a: GValue,
    b: GValue,
    seen: MutableSet<Pair<GValue, GValue>>,
): Boolean = seen.add(a to b)

/** `===` and `!==`: reference identity on class instances; scalars compare
 * as their primitive values. */
context(r: Raise<GuestError>)
public fun valueIdentical(
    a: GValue,
    b: GValue,
): Boolean =
    when {
        isScalar(a) && isScalar(b) -> valueEquals(a, b)
        a is GValue.VNull || b is GValue.VNull -> a is GValue.VNull && b is GValue.VNull
        a is GValue.VUnit || b is GValue.VUnit -> a is GValue.VUnit && b is GValue.VUnit
        else -> a === b
    }

private fun isScalar(value: GValue): Boolean =
    value is GValue.VInt || value is GValue.VLong || value is GValue.VDouble ||
        value is GValue.VBool || value is GValue.VString

context(r: Raise<GuestError>)
private fun mapEquals(
    a: GValue.VMap,
    b: GValue.VMap,
): Boolean {
    if (a.entries.size != b.entries.size) return false
    return a.entries.all { (key, value) ->
        b.entries.containsKey(key) && valueEquals(value, b.entries.getValue(key))
    }
}

/** Forces a lazy list into an ordinary list node chain; ordinary lists come
 * back unchanged, so callers may pass either shape. */
context(r: Raise<GuestError>)
public fun materialize(value: GValue): GValue {
    if (value !is GValue.VLazyList) return value
    val items = mutableListOf<GValue>(value.head)
    var cursor = value.tail
    while (true) {
        val next = forceThunk(cursor)
        when (next) {
            is GValue.VLazyList -> {
                items.add(next.head)
                cursor = next.tail
            }

            is GValue.VList -> {
                items.addAll(next.items)
                return GValue.VList(items, mutable = false)
            }

            else -> {
                return GValue.VList(items, mutable = false)
            }
        }
    }
}

/** Memoized forcing: a thunk computes at most once (4.1 invariant). */
context(r: Raise<GuestError>)
public fun forceThunk(thunk: GValue.VThunk): GValue {
    thunk.counters?.let { it.attempts++ }
    val state = thunk.state
    if (state is ThunkState.Forced) return state.value
    val delayed = state as ThunkState.Delayed
    thunk.counters?.let { it.computations++ }
    val value = delayed.compute(r, emptyList())
    thunk.state = ThunkState.Forced(value)
    return value
}
