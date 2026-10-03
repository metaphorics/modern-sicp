// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.37: the preserving mechanism and the stack
//  waste it avoids. The exercise turns the preserving knob through its
// three settings -- liveness, always, never -- and reads the waste off
// the emitted statements: `always` wraps saves the register analysis
// would have skipped.

package sicp.ch5.solutions

import sicp.ch5.CompilerOptions
import sicp.ch5.PreservingMode

/** The waste report over the three preserving settings: one line per
 *  setting, then the verdict every well-formed compilation carries. */
public fun preservingWaste(): List<String> {
    var paired = true
    val lines = mutableListOf<String>()
    for (
    (name, mode) in
    listOf(
        "liveness" to PreservingMode.LIVENESS,
        "always" to PreservingMode.ALWAYS,
        "never" to PreservingMode.NEVER,
    )
    ) {
        val pairs = savePairs(compiledStatements(operandProbeSource, CompilerOptions(preserving = mode)))
        paired = paired && pairs.none { it.restoreIndex <= it.saveIndex }
        lines.add("$name: ${pairs.size} save pairs, ${pairs.count { !it.clobbered }} superfluous")
    }
    lines.add("every save pairs with one restore: $paired")
    return lines
}
