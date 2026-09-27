// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.76

package sicp.ch2.exercises

import kotlin.math.cos
import kotlin.math.sin
import kotlin.math.sqrt

// --- Explicit dispatch: one closed sealed hierarchy, every operation a `when`. ---

public sealed interface RepZ {
    public data class Rect(
        val x: Double,
        val y: Double,
    ) : RepZ

    public data class Polar(
        val mag: Double,
        val ang: Double,
    ) : RepZ
}

/** Adding a type here means a new `RepZ` variant plus a new arm in every
 * one of these; adding an operation means one new total function. */
public fun edRealPart(z: RepZ): Double =
    when (z) {
        is RepZ.Rect -> z.x
        is RepZ.Polar -> z.mag * cos(z.ang)
    }

public fun edMagnitude(z: RepZ): Double =
    when (z) {
        is RepZ.Rect -> sqrt(z.x * z.x + z.y * z.y)
        is RepZ.Polar -> z.mag
    }

// --- Data-directed: a local table, one package installed per representation. ---

public typealias RepHandler = (Double, Double) -> Double

/** A minimal operation-and-type table over this exercise's own plain-double
 * representations, sharing the section's `put`/`get` discipline without
 * needing the runtime `OpTable`'s `Value`-typed handlers. */
public class RepresentationTable {
    private val entries: MutableMap<Pair<String, String>, RepHandler> = HashMap()

    public fun put(
        op: String,
        tag: String,
        handler: RepHandler,
    ) {
        entries[op to tag] = handler
    }

    public fun get(
        op: String,
        tag: String,
    ): RepHandler? = entries[op to tag]
}

/** Adding a type here is a new `install*Representation` function, calling
 * only [RepresentationTable.put] -- no existing package's entries change. */
public fun installRectRepresentation(table: RepresentationTable) {
    table.put("real-part", "rect") { x, _ -> x }
    table.put("magnitude", "rect") { x, y -> sqrt(x * x + y * y) }
}

public fun installPolarRepresentation(table: RepresentationTable) {
    table.put("real-part", "polar") { mag, ang -> mag * cos(ang) }
    table.put("magnitude", "polar") { mag, _ -> mag }
}

private fun dataDirectedProbe(table: RepresentationTable): Pair<Double, Double> {
    val realPart = table.get("real-part", "rect")?.invoke(3.0, 4.0) ?: error("rect real-part not installed")
    val magnitude = table.get("magnitude", "rect")?.invoke(3.0, 4.0) ?: error("rect magnitude not installed")
    return realPart to magnitude
}

// --- Message passing: reuses exercise 2.75's ComplexObject and real-imag object. ---

private fun messagePassingProbe(): Pair<Double, Double> {
    val z = makeFromRealImagMessagePassing(3.0, 4.0)
    val realPart = applyGenericMessagePassing("real-part", z) ?: error("real-part not answered")
    val magnitude = applyGenericMessagePassing("magnitude", z) ?: error("magnitude not answered")
    return realPart to magnitude
}

/**
 * The same `(realPart, magnitude)` pair of `(3, 4)`, served three ways.
 * Explicit dispatch and message passing both need every existing case
 * edited to add one operation (a new `when`/`when` arm per representation
 * for dispatch, a new branch per object for message passing); the table
 * needs none, only new `put` calls, which the additive-install test
 * below proves by pointer identity rather than by absence of a diff.
 */
public fun ex_2_76(): Triple<Pair<Double, Double>, Pair<Double, Double>, Pair<Double, Double>> {
    val explicit = edRealPart(RepZ.Rect(3.0, 4.0)) to edMagnitude(RepZ.Rect(3.0, 4.0))
    val table = RepresentationTable()
    installRectRepresentation(table)
    installPolarRepresentation(table)
    val dataDirected = dataDirectedProbe(table)
    val messagePassing = messagePassingProbe()
    return Triple(explicit, dataDirected, messagePassing)
}
