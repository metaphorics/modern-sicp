// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.75

package sicp.ch2.exercises

import kotlin.math.atan2
import kotlin.math.cos
import kotlin.math.sin
import kotlin.math.sqrt

public fun interface ComplexObject {
    public fun send(op: String): Double?
}

public fun makeFromRealImagMessagePassing(
    x: Double,
    y: Double,
): ComplexObject =
    ComplexObject { op ->
        when (op) {
            "real-part" -> x
            "imag-part" -> y
            "magnitude" -> sqrt(x * x + y * y)
            "angle" -> atan2(y, x)
            else -> null
        }
    }

public fun applyGenericMessagePassing(
    op: String,
    arg: ComplexObject,
): Double? = arg.send(op)

/** Analogous to [makeFromRealImagMessagePassing]: the returned dispatch
 * closes over `mag` and `ang` instead of `x` and `y`. */
public fun makeFromMagAngMessagePassing(
    mag: Double,
    ang: Double,
): ComplexObject =
    ComplexObject { op ->
        when (op) {
            "magnitude" -> mag
            "angle" -> ang
            "real-part" -> mag * cos(ang)
            "imag-part" -> mag * sin(ang)
            else -> null
        }
    }

/** Builds the polar number with magnitude 5 at the angle of `(3, 4)`,
 * then reads its magnitude and angle back through [applyGenericMessagePassing]. */
public fun ex_2_75(): Pair<Double?, Double?> {
    val z = makeFromMagAngMessagePassing(5.0, atan2(4.0, 3.0))
    return applyGenericMessagePassing("magnitude", z) to applyGenericMessagePassing("angle", z)
}
