// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.65a

package sicp.ch4.solutions

import sicp.ch4.QPattern
import sicp.ch4.QVar
import sicp.ch4.QueryDriver

// Exercise 4.65a (added by this edition): deduplicate the wheel listing.
// The duplicates of 4.65 are frames, not answers; the driver's
// deduplicating answer view keeps one line per distinct rendering, so the
// same query over the same data base answers Warbucks once.

/** The deduplicated wheel listing.
 * => one line per wheel */
public fun deduplicatedWheel(): List<String> {
    val driver = QueryDriver.deduplicating(microshaftSystem())
    return answerLines(driver, QPattern(list(sym("wheel"), v("who"))), listOf(v("who")))
}
