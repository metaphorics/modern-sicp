// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.86

package sicp.ch2.exercises

import arrow.core.raise.Raise

/**
 * Exercise 2.86: complex numbers whose real parts, imaginary parts,
 * magnitudes, and angles may themselves be any tower value. The
 * rectangular and polar internal procedures must stop using the host's
 * `+`, `*`, `sin`, and `cos` on doubles and go through the generic
 * operations instead, with generic `sine`, `cosine`, `square`, and
 * `sqrt` installed per level. [ComplexG] is the generic-parts complex
 * number and [addG]/[mulG]/[magnitudeG]/[makeFromMagAngG] its package;
 * the part arithmetic rides the raising dispatch of exercise 2.84, so
 * mixed-level parts combine.
 *
 * A complex number with generic parts.
 */
public data class ComplexG(
    public val re: Num,
    public val im: Num,
)

/** The real level's own arithmetic: raising delivers reals, so the
 * generic-parts operations need `add`/`sub`/`mul`/`div` to answer at
 * `(real, real)` too. */
public fun installRealPackage(table: NumTable) {
    fun Raise<GenError>.twoReals(args: List<Num>): Pair<Double, Double> {
        if (args.size != 2) raise(GenError.BadArgs("real arithmetic", "expected 2 arguments"))
        val a = args[0]
        val b = args[1]
        if (a !is Real || b !is Real) raise(GenError.BadArgs("real arithmetic", "expected two reals"))
        return a.d to b.d
    }
    table.put("add", listOf("real", "real")) { args ->
        val (a, b) = twoReals(args)
        Real(a + b)
    }
    table.put("sub", listOf("real", "real")) { args ->
        val (a, b) = twoReals(args)
        Real(a - b)
    }
    table.put("mul", listOf("real", "real")) { args ->
        val (a, b) = twoReals(args)
        Real(a * b)
    }
    table.put("div", listOf("real", "real")) { args ->
        val (a, b) = twoReals(args)
        if (b == 0.0) raise(GenError.DivideByZero("real div"))
        Real(a / b)
    }
}

/** Installs the transcendentals the polar path needs, per level: the
 * answers are reals, since trigonometry over exact levels leaves it. */
public fun installTranscendentals(table: NumTable) {
    fun realHandlers(
        name: String,
        f: (Double) -> Double,
    ) {
        for (tag in listOf("integer", "rational", "real")) {
            table.put(name, listOf(tag)) { args ->
                val z = args.single()
                Real(
                    when (z) {
                        is ZLong -> f(z.n.toDouble())
                        is QRat -> f(z.num.toDouble() / z.den.toDouble())
                        is Real -> f(z.d)
                        else -> raise(GenError.BadArgs(name, "not a number level"))
                    },
                )
            }
        }
    }
    realHandlers("sine") { kotlin.math.sin(it) }
    realHandlers("cosine") { kotlin.math.cos(it) }
    realHandlers("square") { it * it }
    realHandlers("sqrt") { kotlin.math.sqrt(it) }
}

context(r: Raise<GenError>)
internal fun genericUnary(
    table: NumTable,
    op: String,
    z: Num,
): Num = applyGeneric(table, op, listOf(z))

/** Reads a number-level part as the host real the angles need. */
context(r: Raise<GenError>)
internal fun partToDouble(z: Num): Double =
    when (z) {
        is ZLong -> z.n.toDouble()
        is QRat -> z.num.toDouble() / z.den.toDouble()
        is Real -> z.d
        else -> r.raise(GenError.BadArgs("angle", "part is not a number level"))
    }

/** Adds two generic-parts complexes: the parts combine through the
 * generic `add`. */
context(r: Raise<GenError>)
public fun addG(
    table: NumTable,
    a: ComplexG,
    b: ComplexG,
): ComplexG = ComplexG(addRaising(table, a.re, b.re), addRaising(table, a.im, b.im))

/** Multiplies two generic-parts complexes: magnitude times magnitude,
 * angle plus angle, all through the generic operations. */
context(r: Raise<GenError>)
public fun mulG(
    table: NumTable,
    a: ComplexG,
    b: ComplexG,
): ComplexG {
    val mag = mulRaising(table, magnitudeG(table, a), magnitudeG(table, b))
    val ang = addRaising(table, angleG(table, a), angleG(table, b))
    return makeFromMagAngG(table, mag, ang)
}

/** The generic magnitude: `sqrt(square(re) + square(im))`. */
context(r: Raise<GenError>)
public fun magnitudeG(
    table: NumTable,
    z: ComplexG,
): Num {
    val re2 = genericUnary(table, "square", z.re)
    val im2 = genericUnary(table, "square", z.im)
    return genericUnary(table, "sqrt", addRaising(table, re2, im2))
}

/** The generic angle: `atan` of the part quotient, answered as a real. */
context(r: Raise<GenError>)
public fun angleG(
    table: NumTable,
    z: ComplexG,
): Num = Real(kotlin.math.atan2(partToDouble(z.im), partToDouble(z.re)))

/** The generic `make-from-mag-ang`:
 * `re = mag*cosine(ang)`, `im = mag*sine(ang)`. */
context(r: Raise<GenError>)
public fun makeFromMagAngG(
    table: NumTable,
    mag: Num,
    ang: Num,
): ComplexG {
    val cosine = genericUnary(table, "cosine", ang)
    val sine = genericUnary(table, "sine", ang)
    return ComplexG(mulRaising(table, mag, cosine), mulRaising(table, mag, sine))
}

/** Runs the exercise's checks: rational parts add exactly, a magnitude
 * over integer parts comes out as a real, and `make-from-mag-ang`
 * rebuilds a complex from generic magnitude and angle. */
public fun ex_2_86(): Boolean {
    val table = NumTable()
    installGenericArithmetic(table)
    installRaise(table)
    installRealPackage(table)
    installTranscendentals(table)
    return arrow.core.raise
        .either {
            val sum = addG(table, ComplexG(ZLong(1), qr(1, 2)), ComplexG(ZLong(2), qr(1, 2)))
            val mag = magnitudeG(table, ComplexG(ZLong(3), ZLong(4)))
            val rebuilt = makeFromMagAngG(table, ZLong(2), ZLong(0))
            val product = mulG(table, ComplexG(ZLong(2), ZLong(0)), ComplexG(ZLong(3), ZLong(0)))
            sum == ComplexG(ZLong(3), qr(1, 1)) &&
                mag == Real(5.0) &&
                rebuilt == ComplexG(Real(2.0), Real(0.0)) &&
                product == ComplexG(Real(6.0), Real(0.0))
        }.getOrNull() ?: false
}
