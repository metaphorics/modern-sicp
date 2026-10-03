// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.67

package sicp.ch4.solutions

import sicp.ch4.QFact
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QVar
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver

// Exercise 4.67: the loop detector. The book's looping examples loop by
// re-entering a rule before its answer exists. The driver's loop-detecting
// constructor bounds each rule chain at an explicit depth, so the looping
// queries terminate: the married loop answers its one fact, and the stock
// queries answer exactly as under the streaming driver. The married
// multiplicity under the detector is engine detail, so the probe pins
// the distinct bindings, which the loop cannot change.

// Exercise 4.67: the detector bounds the loop; answers stay put.

/** The looping married rule of the exercise: a symmetric re-entry. */
internal fun addMarriedLoop(db: QueryDatabase) {
    db.assertFact(QFact(list(sym("married"), sym("Mickey"), sym("Minnie"))))
    db.addRule(
        QRule(
            list(sym("married"), v("x"), v("y")),
            QPattern(list(sym("married"), v("y"), v("x"))),
        ),
    )
}

/** The loop-detector demonstrations. */
public fun loopDetectorDemos(): List<String> {
    val marriedDb = QueryDatabase()
    addMarriedLoop(marriedDb)
    val marriedDriver = QueryDriver.loopDetecting(marriedDb, 8)
    val married = answerLines(marriedDriver, QPattern(list(sym("married"), sym("Mickey"), v("who"))), listOf(v("who")))
    val distinct = married.toSet().sorted()
    val stock = microshaftSystem()
    val stockDriver = QueryDriver.loopDetecting(stock, 8)
    val streaming = QueryDriver.streaming(stock)
    val wheelStock = answerLines(stockDriver, QPattern(list(sym("wheel"), v("who"))), listOf(v("who")))
    val wheelPlain = answerLines(streaming, QPattern(list(sym("wheel"), v("who"))), listOf(v("who")))
    val outranked =
        answerLines(stockDriver, QPattern(list(sym("outranked-by"), list(sym("Bitdiddle"), sym("Ben")), v("boss"))), listOf(v("boss")))
    return listOf(
        "married Mickey ?who under the detector: loop bounded at depth 8, terminates",
        "distinct bindings: $distinct",
        "wheel identical to stock as sets: ${wheelStock.toSet() == wheelPlain.toSet()}",
    ) + outranked
}
