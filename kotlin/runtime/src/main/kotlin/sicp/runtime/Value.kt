// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

package sicp.runtime

import arrow.core.raise.Raise
import kotlinx.collections.immutable.PersistentList
import kotlinx.collections.immutable.persistentListOf
import kotlinx.collections.immutable.toPersistentList

/**
 * The operation shape the dispatch table of 2.4, the primitive registry of
 * 4.1, and the machine operations of chapter 5 all install: applied to
 * argument values, it yields one value or raises a [SchemeError].
 */
public typealias Op = Raise<SchemeError>.(List<Value>) -> Value

/**
 * The dynamic datum: every value the interpreted language passes around.
 * `equals` is the book's `eq?` — identity for the mutable and procedural
 * shapes, content for the atoms; [equalv] is the book's `equal?` and
 * compares structure. `toString` renders the book's surface syntax.
 */
public sealed interface Value

/** An exact integer; checked arithmetic past the `Long` width raises
 * [SchemeError.Overflow]. */
@JvmInline
public value class VInt(
    public val n: Long,
) : Value {
    public override fun toString(): String = n.toString()
}

/** An inexact real. */
@JvmInline
public value class VReal(
    public val d: Double,
) : Value {
    public override fun toString(): String = d.toString()
}

/** The book's `#t` and `#f`. */
@JvmInline
public value class VBool(
    public val b: Boolean,
) : Value {
    public override fun toString(): String = if (b) "#t" else "#f"
}

/** A symbol. */
@JvmInline
public value class VSym(
    public val name: String,
) : Value {
    public override fun toString(): String = name
}

/** A double-quoted string. */
@JvmInline
public value class VStr(
    public val s: String,
) : Value {
    public override fun toString(): String =
        s
            .fold(StringBuilder("\"")) { acc, c ->
                if (c == '"' || c == '\\') acc.append('\\')
                acc.append(c)
            }.append('"')
            .toString()
}

/** The empty list; also the book's unspecified value. */
public data object VNil : Value {
    public override fun toString(): String = "()"
}

/** A cons cell: the mutable pair of 3.3.1. `equals` is identity, the book's
 * `eq?` on pairs; [equalv] compares structure. */
public class VPair(
    car: Value,
    cdr: Value,
) : Value {
    /** The car slot; `setCar` is the book's `set-car!`. */
    public var car: Value = car
        internal set

    /** The cdr slot; `setCdr` is the book's `set-cdr!`. */
    public var cdr: Value = cdr
        internal set

    public override fun toString(): String {
        val out = StringBuilder("(").append(car)
        var cursor = cdr
        while (true) {
            when (cursor) {
                is VNil -> {
                    return out.append(')').toString()
                }

                is VPair -> {
                    out.append(' ').append(cursor.car)
                    cursor = cursor.cdr
                }

                else -> {
                    return out
                        .append(" . ")
                        .append(cursor)
                        .append(')')
                        .toString()
                }
            }
        }
    }
}

/** The book's `(cons x y)`. */
public fun cons(
    car: Value,
    cdr: Value,
): VPair = VPair(car, cdr)

/** The book's `set-car!`: mutates the shared cell. */
public fun VPair.setCar(v: Value) {
    car = v
}

/** The book's `set-cdr!`: mutates the shared cell. */
public fun VPair.setCdr(v: Value) {
    cdr = v
}

/** The book's `car` over a dynamic value. */
context(r: Raise<SchemeError>)
public fun car(v: Value): Value =
    when (v) {
        is VPair -> v.car
        else -> r.raise(SchemeError.TypeMismatch("car of a non-pair: $v"))
    }

/** The book's `cdr` over a dynamic value. */
context(r: Raise<SchemeError>)
public fun cdr(v: Value): Value =
    when (v) {
        is VPair -> v.cdr
        else -> r.raise(SchemeError.TypeMismatch("cdr of a non-pair: $v"))
    }

/** A tag datum plus its payload (2.4.2): the dispatch tag the operation
 * table looks up. */
public data class VTagged(
    val tag: String,
    val data: Value,
) : Value {
    public override fun toString(): String = if (data is VNil) "($tag)" else "($tag $data)"
}

/** A primitive procedure installed under a name. `equals` compares the
 * name, matching the book's printed form. */
public class VPrimitive(
    /** The printed name, e.g. `+`. */
    public val name: String,
    /** The body. */
    public val f: Op,
) : Value {
    public override fun equals(other: Any?): Boolean = other is VPrimitive && other.name == name

    public override fun hashCode(): Int = name.hashCode()

    public override fun toString(): String = "#[primitive $name]"
}

/** A compound procedure: the parameter names, an optional `.`-style rest
 * parameter, the analyzed body forms, and the environment the `lambda`
 * captured (the 3.2 and 4.1 procedure object). */
public class VProc(
    /** The required parameter names, in order. */
    public val params: PersistentList<String>,
    /** The rest parameter after `.`, if the form has one. */
    public val rest: String?,
    /** The body expressions; the last one's value is the answer. */
    public val body: PersistentList<Expr>,
    /** The environment captured at `lambda` time. */
    public val env: Env,
) : Value {
    public override fun toString(): String = "#[compound-procedure]"
}

/** The thunk state of 4.2: a delayed expression and its environment, or
 * the value forcing produced. */
public sealed interface ThunkState {
    /** Delayed, not yet forced. */
    public data class Delayed(
        val expr: Expr,
        val env: Env,
    ) : ThunkState

    /** The memoized result: forcing ran once, and its side effects ran
     * once, which is what exercise 4.27 probes. */
    public data class Forced(
        val v: Value,
    ) : ThunkState
}

/** A memoized thunk of the lazy evaluator (4.2). The state is a `var` so
 * forcing installs the answer in the shared object. */
public class VThunk(
    /** The current state; `forceIt` flips [ThunkState.Delayed] to
     * [ThunkState.Forced]. */
    public var state: ThunkState,
) : Value {
    public override fun toString(): String = "#[thunk]"
}

/** The unmemoized probe variant of exercises 4.27 and 4.29: every force
 * re-runs the delayed expression. */
public class VThunkNoMemo(
    /** The delayed expression. */
    public val expr: Expr,
    /** The environment to evaluate it in. */
    public val env: Env,
) : Value {
    public override fun toString(): String = "#[thunk no-memo]"
}

/** A compiled procedure (5.5.7): an entry label resolved against the
 * assembled machine's label table, the parameter names, and the
 * environment installed as the frame chain. */
public class VCompiledProc(
    /** The entry label name in the machine's label table. */
    public val entry: String,
    /** The parameter names, bound to the argument values in order. */
    public val params: PersistentList<String>,
    /** The environment the procedure's frame extends. */
    public val env: Env,
) : Value {
    public override fun toString(): String = "#[compiled-procedure $entry]"
}

/** The book's `(delay expr)` under `env` as a memoized [VThunk]. */
public fun delayIt(
    expr: Expr,
    env: Env,
): VThunk = VThunk(ThunkState.Delayed(expr, env))

/** The book's `force-it`: a [VThunk] runs `eval` once and memoizes, a
 * [VThunkNoMemo] re-runs every time, anything else is already a value.
 * `eval` runs outside the state write, so a thunk may legally touch other
 * thunks while forcing; a failed force leaves the thunk delayed. */
context(r: Raise<SchemeError>)
public fun forceIt(
    thunk: Value,
    eval: Raise<SchemeError>.(Expr, Env) -> Value,
): Value =
    when (thunk) {
        is VThunk -> {
            when (val s = thunk.state) {
                is ThunkState.Forced -> {
                    s.v
                }

                is ThunkState.Delayed -> {
                    val v = eval(r, s.expr, s.env)
                    thunk.state = ThunkState.Forced(v)
                    v
                }
            }
        }

        is VThunkNoMemo -> {
            eval(r, thunk.expr, thunk.env)
        }

        else -> {
            thunk
        }
    }

/** The book's `equal?`: structural comparison over data, `eq?` for the
 * procedural shapes. */
public fun equalv(
    a: Value,
    b: Value,
): Boolean =
    when {
        a is VPair && b is VPair -> equalv(a.car, b.car) && equalv(a.cdr, b.cdr)
        else -> a == b
    }

/** Builds a proper list out of `items`, [VNil]-terminated. */
public fun vlist(items: List<Value>): Value = items.foldRight(VNil as Value) { v, acc -> cons(v, acc) }

/** Builds a proper list out of `items`, [VNil]-terminated. */
public fun vlist(vararg items: Value): Value = vlist(items.toList())

/** The items of a proper list, in order. */
context(r: Raise<SchemeError>)
public fun listItems(v: Value): PersistentList<Value> {
    val items = mutableListOf<Value>()
    var cursor = v
    while (cursor is VPair) {
        items.add(cursor.car)
        cursor = cursor.cdr
    }
    if (cursor is VNil) return items.toPersistentList()
    r.raise(SchemeError.TypeMismatch("not a proper list: $v"))
}

/** Applies a primitive procedure to `args`. */
context(r: Raise<SchemeError>)
public fun callPrimitive(
    v: Value,
    args: List<Value>,
): Value =
    when (v) {
        is VPrimitive -> v.f(r, args)
        else -> r.raise(SchemeError.NotApplicable(v))
    }

/** The book's `#t`/`#f` test: only [VBool]`(false)` is false. */
public fun isTrue(v: Value): Boolean = v != VBool(false)

/** The empty parameter list shared by zero-argument procedures. */
public val noParams: PersistentList<String> = persistentListOf()
