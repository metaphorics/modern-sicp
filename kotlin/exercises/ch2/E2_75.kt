// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.75

package sicp.ch2.exercises

import kotlin.math.atan2
import kotlin.math.sqrt

/**
 * Exercise 2.75: implement `makeFromMagAng` in message-passing style,
 * analogous to `makeFromRealImagMessagePassing` given below (the
 * section's own message-passing `make-from-real-imag`).
 *
 * The scaffold returns `(magnitude, angle)` read back from a polar
 * message object built from `real-part`/`imag-part` values, through
 * [applyGenericMessagePassing].
 */
public fun interface ComplexObject {
    public fun send(op: String): Double?
}

/** Given by the section: the message-passing `make-from-real-imag`. */
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

/** Given by the section: feeds the operation name to the object and lets it answer. */
public fun applyGenericMessagePassing(
    op: String,
    arg: ComplexObject,
): Double? = arg.send(op)

public fun ex_2_75(): Pair<Double?, Double?> = throw PendingExercise()
