// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.55

package sicp.ch4.solutions

import sicp.ch4.QPattern
import sicp.ch4.QVar
import sicp.ch4.QueryDriver

// Exercise 4.55: simple queries over the Microshaft data base. The three
// queries of the exercise are typed patterns; the streaming driver answers
// every match in data-base order, and the strict matcher of the query
// engine matches only the two-element accounting job in (b) while the
// dotted tail of (c) reaches the whole rest of the address.

/** The three simple queries with their answers, in data-base order. */
public fun simpleQueries(): List<String> {
    val db = microshaftSystem()
    val driver = QueryDriver.streaming(db)
    val name = listOf(v("name"))
    val title = listOf(v("name"), v("title"))
    val where = listOf(v("name"), v("where"))
    return answerLines(driver, QPattern(list(sym("supervisor"), v("name"), list(sym("Bitdiddle"), sym("Ben")))), name) +
        answerLines(driver, QPattern(list(sym("job"), v("name"), improper(listOf(sym("accounting")), v("title")))), title) +
        answerLines(driver, QPattern(list(sym("address"), v("name"), improper(listOf(sym("Slumerville")), v("where")))), where)
}
