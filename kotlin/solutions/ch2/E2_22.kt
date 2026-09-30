// SPDX-License-Identifier: GPL-3.0-only
// Chapter 2, exercise 2.22

package sicp.ch2.exercises

import sicp.runtime.Datum
import sicp.runtime.Empty
import sicp.runtime.Whole
import sicp.runtime.datumList
import sicp.runtime.pair
import sicp.runtime.renderDatum

private fun square(v: Datum): Datum = Whole(wholeNumber(v) * wholeNumber(v))

/** Reverse accumulation exposes the order error in the iterative construction. */
public fun squareListIter(items: Datum): Datum {
    tailrec fun iter(
        things: Datum,
        answer: Datum,
    ): Datum = if (things === Empty) answer else iter(secondPart(things), pair(square(firstPart(things)), answer))

    return iter(items, Empty)
}

/** Swapping pair arguments builds nested improper cells rather than a sequence. */
public fun squareListIterSwapped(items: Datum): Datum {
    tailrec fun iter(
        things: Datum,
        answer: Datum,
    ): Datum = if (things === Empty) answer else iter(secondPart(things), pair(answer, square(firstPart(things))))

    return iter(items, Empty)
}

/** Canonical native rendering of Louis's first construction attempt. */
public fun ex_2_22(): String = renderDatum(squareListIter(datumList(Whole(1L), Whole(2L), Whole(3L), Whole(4L))))
