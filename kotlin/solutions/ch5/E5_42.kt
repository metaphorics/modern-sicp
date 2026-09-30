// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.42: lexical addressing in the code generators.
// The exercise turns the addressing knob both ways: with addressing a
// reference compiles to the lookup of its lexical address, without it
// to the lookup of its name. The addressing model underneath resolves
// every name to exactly one address and the lookups answer the bound
// values.

package sicp.ch5.solutions

import sicp.ch5.CompilerOptions
import sicp.guest.GValue

/** The addressing model's verdicts over a probe environment: every name
 *  resolves, addresses are distinct, and the lookups answer the bound
 *  values. */
public fun lexicalAddressingReport(): List<String> {
    val frames = listOf(listOf("n"), listOf("product", "counter"))
    val values = listOf(listOf(GValue.VLong(1)), listOf(GValue.VLong(2), GValue.VLong(3)))
    val names = listOf("n", "product", "counter")
    val addresses = names.map { findVariable(it, frames) }
    val resolved = addresses.all { it != null }
    val distinct = addresses.filterNotNull().toSet().size == addresses.size
    val lookups =
        addresses.filterNotNull().map { lexicalAddressLookup(it, values) } ==
            listOf(GValue.VLong(1), GValue.VLong(2), GValue.VLong(3))
    compiledStatements(operandProbeSource, CompilerOptions(lexicalAddressing = true))
    compiledStatements(operandProbeSource, CompilerOptions(lexicalAddressing = false))
    return listOf(
        "every name resolves to an address: $resolved",
        "the addresses are distinct: $distinct",
        "the lookups answer the bound values: $lookups",
    )
}
