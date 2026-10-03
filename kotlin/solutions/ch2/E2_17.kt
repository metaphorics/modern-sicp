// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.17

package sicp.ch2.exercises

import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.PairCell
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.renderDatum

/** Typed selectors shared by the sequence exercises. */
public fun firstPart(v: Datum): Datum = (v as PairCell).first

public fun secondPart(v: Datum): Datum = (v as PairCell).second

public fun wholeNumber(v: Datum): Long = (v as Whole).value

public fun lastPair(items: Datum): Datum = if (secondPart(items) === Empty) items else lastPair(secondPart(items))

/** The canonical native rendering of the final one-element datum pair. */
public fun ex_2_17(): String = renderDatum(lastPair(datumList(Whole(23L), Whole(72L), Whole(149L), Whole(34L))))
