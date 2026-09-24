// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.76a

package sicp.ch2.exercises

import kotlin.math.abs
import kotlin.math.cos

/**
 * A third representation for exercise 2.76's table, installed entirely
 * from this file: the angle is stored in degrees rather than radians.
 * Neither [RepresentationTable] nor [installRectRepresentation]/
 * [installPolarRepresentation] change; [RepZ], the explicit-dispatch
 * sealed hierarchy from the same exercise, could not gain this
 * representation the same way, since a new subtype there edits every
 * exhaustive `when` built over it.
 */
public fun installPolarDegRepresentation(table: RepresentationTable) {
    table.put("real-part", "polar-deg") { mag, angDeg -> mag * cos(Math.toRadians(angDeg)) }
    table.put("magnitude", "polar-deg") { mag, _ -> mag }
}

/**
 * Installs rectangular and polar (exercise 2.76's own packages), takes a
 * pointer to the rectangular `real-part` handler, installs the degrees
 * package from this file, and confirms both that the earlier pointer is
 * untouched and that the new package computes the book's own trigonometric
 * relation correctly.
 */
public fun ex_2_76a(): Boolean {
    val table = RepresentationTable()
    installRectRepresentation(table)
    installPolarRepresentation(table)
    val rectRealPartBefore = table.get("real-part", "rect")
    installPolarDegRepresentation(table)
    val rectRealPartAfter = table.get("real-part", "rect")

    val degResult = table.get("real-part", "polar-deg")?.invoke(5.0, 60.0)
    val expected = 5.0 * cos(Math.PI / 3)

    return rectRealPartBefore === rectRealPartAfter &&
        degResult != null &&
        abs(degResult - expected) < 1e-9
}
