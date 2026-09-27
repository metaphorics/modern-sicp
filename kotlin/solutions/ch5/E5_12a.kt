// SPDX-License-Identifier: GPL-3.0-only
// Chapter 5, exercise 5.12a: the per-type instruction census, this
// edition's extension of the 5.12 assembler summary. The census line
// counts the assembled instructions by kind -- assign, test, branch, goto,
// save, restore, perform -- printed alongside the register and label
// summaries.

package sicp.ch5.solutions

import sicp.ch5.renderAssemblySummary

/** The census line of the gcd machine's summary: the counts by type the
 *  5.12a assembler gathers. */
public fun gcdMachineCensus(): String = renderAssemblySummary(gcdSummary()).lines().last()
