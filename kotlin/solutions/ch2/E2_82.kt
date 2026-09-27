// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.82

package sicp.ch2.exercises

import arrow.core.raise.Raise

/**
 * Exercise 2.82: generalize `apply-generic` to coerce in the multi-
 * argument case by trying, in turn, to coerce every argument to the type
 * of the first argument, then to the type of the second, and so on.
 * [applyGenericMultiCoerce] implements that strategy. The exercise then
 * asks where it is not general enough: [ex_2_82] shows a table holding a
 * suitable mixed-type operation that the strategy never tries, because it
 * coerces all arguments uniformly or not at all -- never a subset.
 *
 * The strategy of the exercise: on a miss, try coercing all arguments
 * into each argument's type in turn, retrying after each uniform
 * coercion.
 */
context(r: Raise<GenError>)
public fun applyGenericMultiCoerce(
    table: NumTable,
    coercions: CoercionTable,
    op: String,
    args: List<Num>,
): Num {
    table.get(op, args.map { typeTagOf(it) })?.let { return it.invoke(r, args) }
    for (target in args) {
        val targetTag = typeTagOf(target)
        var all = true
        val coerced =
            args.map { arg ->
                if (typeTagOf(arg) == targetTag) {
                    arg
                } else {
                    val step = coercions.getCoercion(arg::class, target::class)
                    if (step == null) {
                        all = false
                        arg
                    } else {
                        step(arg)
                    }
                }
            }
        if (all) {
            table.get(op, coerced.map { typeTagOf(it) })?.let { return it.invoke(r, coerced) }
        }
    }
    return r.raise(GenError.NoMethod(op, args.map { typeTagOf(it) }))
}

/** Coercions up the tower front, enough for the demonstration. */
public fun installTowerCoercions(coercions: CoercionTable) {
    coercions.putCoercion(ZLong::class, QRat::class) { z -> QRat((z as ZLong).n.toBigInteger(), java.math.BigInteger.ONE) }
    coercions.putCoercion(QRat::class, Real::class) { z ->
        val q = z as QRat
        Real(q.num.toDouble() / q.den.toDouble())
    }
    coercions.putCoercion(Real::class, Complex::class) { z -> Complex(Rect((z as Real).d, 0.0)) }
    coercions.putCoercion(ZLong::class, Real::class) { z -> Real((z as ZLong).n.toDouble()) }
    coercions.putCoercion(ZLong::class, Complex::class) { z -> Complex(Rect((z as ZLong).n.toDouble(), 0.0)) }
}

/** The demonstration table: `blend` accepts `(real real real)`
 * uniformly, and `mix` accepts `(rational real)` -- a mixed-type
 * operation sitting in the table. */
public fun installBlendOperations(table: NumTable) {
    table.put("blend", listOf("real", "real", "real")) { args ->
        val (a, b, c) = args.map { (it as Real).d }
        Real(a + b + c)
    }
    table.put("mix", listOf("rational", "real")) { args ->
        val b = args[0] as QRat
        val c = args[1] as Real
        Real(b.num.toDouble() / b.den.toDouble() + c.d)
    }
}

/** Runs the two observations: the strategy finds the uniform `(real real
 * real)` operation for three coercible integers, but for `mix` on
 * `(integer real)` it never tries the `(rational real)` operation,
 * although the coercion `integer -> rational` is installed -- coercing
 * only the first argument, a subset, is not among its moves. Returns
 * `found uniform` to `subset-mixed-op-unreachable`. */
public fun ex_2_82(): Pair<Boolean, Boolean> {
    val table = NumTable()
    installBlendOperations(table)
    val coercions = CoercionTable()
    installTowerCoercions(coercions)
    val uniform =
        arrow.core.raise.either {
            applyGenericMultiCoerce(table, coercions, "blend", listOf(ZLong(1), Real(2.0), ZLong(3)))
        }
    val subset =
        arrow.core.raise.either {
            applyGenericMultiCoerce(table, coercions, "mix", listOf(ZLong(1), Real(2.0)))
        }
    val uniformFound = uniform.getOrNull() == Real(6.0)
    val subsetOpUnreachable = subset.leftOrNull() is GenError.NoMethod
    return uniformFound to subsetOpUnreachable
}
