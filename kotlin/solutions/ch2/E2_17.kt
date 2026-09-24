// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.17

package sicp.ch2.exercises

import sicp.runtime.VInt
import sicp.runtime.VNil
import sicp.runtime.VPair
import sicp.runtime.Value
import sicp.runtime.vlist

/**
 * The section's basic accessors over the chain representation, shared by
 * the sequence exercises: [carOf] and [cdrOf] select the head and the
 * rest of a chain, [numOf] reads a chain element's integer. The runtime's
 * own `car` and `cdr` raise a typed error on a non-pair; these exercises
 * predate chapter 4's error machinery, so they cast directly.
 */
public fun carOf(v: Value): Value = (v as VPair).car

public fun cdrOf(v: Value): Value = (v as VPair).cdr

public fun numOf(v: Value): Long = (v as VInt).n

/**
 * The book's `last-pair`: cdrs down the chain until the rest is the
 * end-of-list marker, then returns the final one-element list.
 */
public fun lastPair(items: Value): Value = if (cdrOf(items) is VNil) items else lastPair(cdrOf(items))

/** `lastPair` of the book's `(23 72 149 34)`, printed. */
public fun ex_2_17(): String = lastPair(vlist(VInt(23L), VInt(72L), VInt(149L), VInt(34L))).toString()
