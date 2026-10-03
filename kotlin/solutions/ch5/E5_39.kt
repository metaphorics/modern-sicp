// SPDX-License-Identifier: GPL-3.0-only
// Original exercise
//
// Chapter 5, exercise 5.39: the machine operation `lexical-address-lookup`.
// A lexical address is the compile-time environment's own coordinate: a
// frame position and a displacement. The lookup walks the frame list the
// compiler threads and answers the bound value or the typed unassigned
// word for an address that names nothing.

package sicp.ch5.solutions

import sicp.guest.GValue

/** A lexical address: frame position and displacement. */
public data class LexicalAddress(
    val frame: Int,
    val offset: Int,
)

/** The compile-time environment: frames of names, innermost first, the
 *  shape the compiler threads. */
public fun lexicalEnvironment(frames: List<List<String>>): List<List<String>> = frames

/** `lexical-address-lookup`: the value the address names in [values],
 *  or the unassigned word when the address names nothing. */
public fun lexicalAddressLookup(
    address: LexicalAddress,
    values: List<List<GValue>>,
): GValue =
    values
        .getOrNull(address.frame)
        ?.getOrNull(address.offset)
        ?: GValue.VUnassigned
