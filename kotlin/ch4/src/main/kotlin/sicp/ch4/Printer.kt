// SPDX-License-Identifier: GPL-3.0-only
// The printer of section 4.1 (the given-code printing half of D23),
// implementing spec/scheme-subset/printer.md: booleans `#t`/`#f`, bare
// symbols, double-quoted strings, dotted pairs, `()`, floats as the shortest
// round-trip decimal with a mandatory point and the mantissa-exponent form
// outside [1e-6, 1e21), and the `#[primitive-procedure name]` and
// `#[compound-procedure name]` shapes.

package sicp.ch4

import sicp.runtime.VBool
import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.VPrimitive
import sicp.runtime.VProc
import sicp.runtime.VReal
import sicp.runtime.VStr
import sicp.runtime.VSym
import sicp.runtime.VTagged
import sicp.runtime.Value
import kotlin.math.abs

/**
 * The printed form of a value: strings quoted and escaped, per the printer
 * contract. This is the form a top-level result line and every irritant use.
 */
public fun printValue(v: Value): String = render(v, displayStrings = false)

/**
 * The `display` form of a value: identical to [printValue] except that a
 * string prints bare, without quotes or escapes.
 */
public fun displayValue(v: Value): String = render(v, displayStrings = true)

private fun render(
    v: Value,
    displayStrings: Boolean,
): String =
    when (v) {
        is VInt -> {
            v.n.toString()
        }

        is VReal -> {
            printFloat(v.d)
        }

        is VBool -> {
            if (v.b) "#t" else "#f"
        }

        is VSym -> {
            v.name
        }

        is VStr -> {
            if (displayStrings) v.s else escapeString(v.s)
        }

        is VNil -> {
            "()"
        }

        is VPrimitive -> {
            "#[primitive-procedure ${v.name}]"
        }

        is VProc -> {
            v.toString()
        }

        is VPair -> {
            renderPair(v, displayStrings)
        }

        is VTagged -> {
            if (v.data is VNil) {
                "(${v.tag})"
            } else {
                "(${v.tag} ${render(v.data, displayStrings)})"
            }
        }

        else -> {
            v.toString()
        }
    }

private fun renderPair(
    v: VPair,
    displayStrings: Boolean,
): String {
    val out = StringBuilder("(").append(render(v.car, displayStrings))
    var cursor = v.cdr
    while (true) {
        when (cursor) {
            is VNil -> {
                return out.append(')').toString()
            }

            is VPair -> {
                out.append(' ').append(render(cursor.car, displayStrings))
                cursor = cursor.cdr
            }

            else -> {
                return out
                    .append(" . ")
                    .append(render(cursor, displayStrings))
                    .append(')')
                    .toString()
            }
        }
    }
}

/** A string between double quotes; only `"` and `\` print escaped. */
private fun escapeString(s: String): String {
    val out = StringBuilder("\"")
    for (c in s) {
        if (c == '"' || c == '\\') out.append('\\')
        out.append(c)
    }
    return out.append('"').toString()
}

/**
 * The printer contract for floats: the shortest decimal string that reads
 * back as the same IEEE 754 double, always with a decimal point; fixed
 * notation in [1e-6, 1e21) and mantissa-exponent form outside it.
 */
public fun printFloat(d: Double): String {
    if (d.isNaN() || d.isInfinite()) throw IllegalArgumentException("no printed form for $d")
    if (d == 0.0) return "0.0"
    val shortest = d.toString() // JDK 19 and later print the shortest round-trip form
    val magnitude = abs(d)
    if (magnitude >= 1e-6 && magnitude < 1e21) {
        return java.math.BigDecimal(shortest).toPlainString()
    }
    val e = shortest.indexOf('E')
    if (e >= 0) return shortest.replace('E', 'e')
    // Unreachable for the host: Double.toString switches to the E form
    // outside [1e-3, 1e7), and both window edges lie inside that range.
    throw IllegalStateException("no mantissa-exponent form for $d")
}
