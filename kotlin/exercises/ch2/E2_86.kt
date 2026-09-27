// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.86

package sicp.ch2.exercises

import sicp.runtime.PendingSolution

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
 */
public fun ex_2_86(): Boolean = throw PendingSolution()
