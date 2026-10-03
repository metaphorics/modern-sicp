// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.12a (added by this edition and extends exercise
// 5.12; SICP numbers stop at 5.12): the assembler's census by instruction
// type. Where 5.12 lists the controller's instruction texts, 5.12a counts
// its instructions per type, in the order the types first appear.

package sicp.ch5.solutions

import sicp.runtime.Label
import sicp.runtime.Stmt

/** The per-type instruction census of [controller]: one `type count` item
 *  per type, in first-appearance order; a label is not an instruction. */
public fun instructionCensus(controller: List<Stmt>): String {
    val counts = linkedMapOf<String, Int>()
    for (instruction in controller) {
        if (instruction is Label) continue
        val kind = instructionKind(instruction)
        counts[kind] = (counts[kind] ?: 0) + 1
    }
    return counts.entries.joinToString(separator = " ", prefix = "(by type ", postfix = ")") { (kind, count) -> "$kind $count" }
}

/** The 5.12a census of the gcd machine: one test, one branch, three
 *  assigns, one goto. */
public fun gcdMachineCensus(): String = instructionCensus(gcdController)
