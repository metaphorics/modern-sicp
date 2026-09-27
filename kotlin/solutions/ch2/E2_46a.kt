// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.46a

package sicp.ch2.exercises

/** `a + b` reads exactly like [addVect]. */
public operator fun Vect.plus(other: Vect): Vect = addVect(this, other)

/** `a - b` reads exactly like [subVect]. */
public operator fun Vect.minus(other: Vect): Vect = subVect(this, other)

/** `a * s` reads exactly like [scaleVect]. */
public operator fun Vect.times(scalar: Double): Vect = scaleVect(this, scalar)

/** `makeVect(1.0, 2.0) + makeVect(3.0, 4.0)`. */
public fun ex_2_46a(): Vect = makeVect(1.0, 2.0) + makeVect(3.0, 4.0)
