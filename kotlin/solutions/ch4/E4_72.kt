// SPDX-License-Identifier: GPL-3.0-only
// Chapter 4, exercise 4.72: appending starves the later disjuncts.

package sicp.ch4.solutions

import sicp.ch4.QFact
import sicp.ch4.QOr
import sicp.ch4.QPattern
import sicp.ch4.QRule
import sicp.ch4.QueryDatabase
import sicp.ch4.QueryDriver
import sicp.ch4.renderAnswer

// Exercise 4.72: interleaving keeps every disjunct alive where appending
// starves the later ones. The symmetric loves rule answers forever under
// the streaming driver. The engine's disjunction interleaves its branch
// streams, so the finite supervisor branch answers every other time.
// Appending the same two branch streams -- all loves answers, then the
// supervisor answers -- is built here from the two separate runs; its
// first branch never ends, so no prefix of any length reaches a
// supervisor. Both branches bind the same variables, so every frame
// answers both.

// Exercise 4.72: interleaving reaches the second disjunct; appending never does.

/** The symmetric loves cycle of the exercise. */
internal fun addLoves(db: QueryDatabase) {
    db.assertFact(QFact(list(sym("loves"), list(sym("Minnie"), sym("Mouse")), list(sym("Mickey"), sym("Mouse")))))
    db.addRule(
        QRule(
            list(sym("loves"), v("x"), v("y")),
            QPattern(list(sym("loves"), v("y"), v("x"))),
        ),
    )
}

/** Four answers each from the interleaved disjunction and from the
 * appended counterfactual, with the supervisor answers each prefix holds. */
public fun interleaveVersusAppend(): List<String> {
    val db = microshaftSystem()
    addLoves(db)
    val driver = QueryDriver.streaming(db)
    val both = listOf(v("a"), v("b"))
    val loves = QPattern(list(sym("loves"), v("a"), v("b")))
    val supervisor = QPattern(list(sym("supervisor"), v("a"), v("b")))
    val interleaved = takeAnswerLines(driver, QOr(listOf(loves, supervisor)), both, 4)
    val appended =
        (driver.run(loves, both) + driver.run(supervisor, both))
            .take(4)
            .flatMap { renderAnswer(it, both) }
            .toList()
    return listOf("interleaved, first 4 answers:") + interleaved +
        listOf("supervisor answers in the interleaved 4: ${supervisorCount(interleaved)}") +
        listOf("appended, first 4 answers:") + appended +
        listOf("supervisor answers in the appended 4: ${supervisorCount(appended)}")
}

/** The supervisor answers among rendered lines: every `?b` that is not
 * one of the loves pair. */
private fun supervisorCount(lines: List<String>): Int = lines.count { line -> line.startsWith("?b = ") && !line.endsWith("Mouse]") }
