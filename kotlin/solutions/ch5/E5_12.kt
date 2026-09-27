// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.12: the assembler's summary. While assembling, the
// assembler can gather data about the controller: how often each
// instruction text occurs, which registers the controller names, and which
// labels it defines. The summary of the book's gcd machine is computed
// here from a fresh assembly of the same controller the 5.7 machine runs.

package sicp.ch5.solutions

import sicp.ch5.AssemblySummary
import sicp.ch5.Machine
import sicp.ch5.assemble
import sicp.ch5.renderAssemblySummary

/** Assembles the gcd controller once more and takes the summary the
 *  assembler gathered. */
internal fun gcdSummary(): AssemblySummary =
    machineRun {
        val machine = Machine(listOf("a", "b", "t"), sicp.ch5.arithOperations)
        assemble(gcdController, machine).summary
    }

/** The 5.12 summary of the gcd machine: the instruction census, the
 *  registers used, the labels. */
public fun gcdMachineSummary(): String = renderAssemblySummary(gcdSummary()).lines().dropLast(1).joinToString("\n")
