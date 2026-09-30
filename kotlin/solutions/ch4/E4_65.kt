// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.65

package sicp.ch4.solutions

import sicp.ch4.QPattern
import sicp.ch4.QVar
import sicp.ch4.QueryDriver

// Exercise 4.65: the wheel query. The prose wheel rule says a wheel is
// anyone who supervises a supervisor; the query answers Warbucks once per
// qualifying chain, so Warbucks arrives four times and Ben once. The
// stream carries every frame the rules produce, and that is the lesson:
// the query is a stream of frames, not a set of answers.

/** The wheel listing with its multiplicities. */
public fun wheelQuery(): List<String> {
    val driver = QueryDriver.streaming(microshaftSystem())
    val who = listOf(v("who"))
    val answers = answerLines(driver, QPattern(list(sym("wheel"), v("who"))), who)
    val warbucks = answers.count { it.contains("Warbucks") }
    val ben = answers.count { it.contains("Bitdiddle") }
    return answers + "Warbucks appears $warbucks times" + "Ben appears $ben times"
}
