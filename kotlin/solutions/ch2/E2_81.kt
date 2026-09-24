// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.81

package sicp.ch2.exercises

import arrow.core.raise.Raise

/**
 * Exercise 2.81: Louis Reasoner notices `apply-generic` may coerce
 * arguments already of the same type, and installs identity coercions to
 * "fix" it.
 *
 * (a) With `complex->complex` installed and `exp` present only for two
 * integers, `exp` on two complex numbers misses, coerces the first
 * argument to its own type -- a no-op -- and retries: the same miss,
 * forever.
 *
 * (b) Louis is wrong: the plain dispatch already answers a same-type miss
 * correctly, by raising `NoMethod`.
 *
 * (c) [applyGenericNoSelfCoercion] skips coercion entirely when both
 * arguments already share a type.
 *
 * Louis's identity coercions, keyed by [KClass] pairs like every entry
 * of the coercion registry.
 */
public fun installIdentityCoercions(coercions: CoercionTable) {
    coercions.putCoercion(ZLong::class, ZLong::class) { z -> z }
    coercions.putCoercion(Complex::class, Complex::class) { z -> z }
}

/** The `exp` operation of the book's setup, present only for two
 * integers. */
public fun installExp(table: NumTable) {
    table.put("exp", listOf("integer", "integer")) { args ->
        val (a, b) = twoNums("exp", args)
        if (a !is ZLong || b !is ZLong) raise(GenError.BadArgs("exp", "expected two integers"))
        ZLong(Math.round(Math.pow(a.n.toDouble(), b.n.toDouble())))
    }
}

/** The book's apply-generic under Louis's coercions, capped at `budget`
 * retries so the same-type loop this exercise predicts stays observable
 * instead of hanging. */
context(r: Raise<GenError>)
public fun applyGenericLouis(
    table: NumTable,
    coercions: CoercionTable,
    op: String,
    args: List<Num>,
    budget: Int,
): Num {
    val tags = args.map { typeTagOf(it) }
    table.get(op, tags)?.let { return it.invoke(r, args) }
    var current = args
    repeat(budget) {
        if (current.size == 2) {
            val t1toT2 = coercions.getCoercion(current[0]::class, current[1]::class)
            val t2toT1 = coercions.getCoercion(current[1]::class, current[0]::class)
            current =
                when {
                    t1toT2 != null -> listOf(t1toT2(current[0]), current[1])
                    t2toT1 != null -> listOf(current[0], t2toT1(current[1]))
                    else -> return r.raise(GenError.NoMethod(op, current.map { typeTagOf(it) }))
                }
            table.get(op, current.map { typeTagOf(it) })?.let { return it.invoke(r, current) }
        } else {
            return r.raise(GenError.NoMethod(op, current.map { typeTagOf(it) }))
        }
    }
    return r.raise(GenError.BadArgs(op, "coercion loop exceeded the budget of $budget retries"))
}

/** Part (c): the revision that refuses to coerce two arguments already of
 * the same type. */
context(r: Raise<GenError>)
public fun applyGenericNoSelfCoercion(
    table: NumTable,
    coercions: CoercionTable,
    op: String,
    args: List<Num>,
): Num {
    val tags = args.map { typeTagOf(it) }
    table.get(op, tags)?.let { return it.invoke(r, args) }
    if (args.size == 2 && tags[0] != tags[1]) {
        val t1toT2 = coercions.getCoercion(args[0]::class, args[1]::class)
        val t2toT1 = coercions.getCoercion(args[1]::class, args[0]::class)
        when {
            t1toT2 != null -> {
                return applyGenericNoSelfCoercion(table, coercions, op, listOf(t1toT2(args[0]), args[1]))
            }

            t2toT1 != null -> {
                return applyGenericNoSelfCoercion(table, coercions, op, listOf(args[0], t2toT1(args[1])))
            }

            else -> {}
        }
    }
    return r.raise(GenError.NoMethod(op, tags))
}

/** Observes all three parts: (a) the loop, capped; (b) the plain dispatch
 * answers correctly by itself; (c) the revision answers the same-type
 * miss with `NoMethod`. */
public fun ex_2_81(): Triple<Boolean, Boolean, Boolean> {
    val table = NumTable()
    installGenericArithmetic(table)
    installExp(table)
    val coercions = CoercionTable()
    installIdentityCoercions(coercions)
    val looped =
        arrow.core.raise.either {
            applyGenericLouis(table, coercions, "exp", listOf(Complex(Rect(2.0, 0.0)), Complex(Rect(3.0, 0.0))), 64)
        }
    val plain =
        arrow.core.raise.either {
            applyGeneric(table, "exp", listOf(Complex(Rect(2.0, 0.0)), Complex(Rect(3.0, 0.0))))
        }
    val fixed =
        arrow.core.raise.either {
            applyGenericNoSelfCoercion(table, coercions, "exp", listOf(Complex(Rect(2.0, 0.0)), Complex(Rect(3.0, 0.0))))
        }
    val loopIsCapped = looped.leftOrNull() is GenError.BadArgs
    val plainIsNoMethod = plain.leftOrNull() is GenError.NoMethod
    val fixedIsNoMethod = fixed.leftOrNull() is GenError.NoMethod
    return Triple(loopIsCapped, plainIsNoMethod, fixedIsNoMethod)
}
